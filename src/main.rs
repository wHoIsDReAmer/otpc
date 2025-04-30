mod commands;
mod config;
mod otp;
mod qrcode;

use clap::Parser as _;
use commands::Command;
use anyhow::Result;

use config::{Account, Config};
use otp::Otp;

fn main() -> Result<()> {
    let mut config = config::get_config()?
        .lock()
        .map_err(|e| anyhow::anyhow!(e.to_string()))?;

    let command = Command::parse();
    match command {
        Command::List => {
            let accounts = config.accounts.clone();
            if accounts.is_empty() {
                println!("No accounts found");
            } else {
                for (i, account) in accounts.iter().enumerate() {
                    println!("{}: name: {}, issuer: {}", (i+1), account.name, account.issuer);
                }
            }
        }
        Command::Code { account, otp_type, counter } => {
            let account = config.accounts.iter().find(|acc| acc.name == account)
                .ok_or(anyhow::anyhow!("Account not found"))?;

            let otp = Otp::new(account.secret.as_str(), 6, 30, otp_type);
            let code = match counter {
                Some(counter) => otp.generate_hotp(counter),
                None => otp.generate_code(),
            };

            println!("{}", code);
        }
        Command::Delete { account } => {
            config.accounts.retain(|acc| acc.name != account);
            let path = Config::get_path()?;
            config.save_to_file(&path)?;

            println!("Account deleted")
        }
        Command::Load { secret, account, issuer } => {
            if config.accounts.iter().any(|acc| acc.name == account) {
                println!("Account already exists");
                return Ok(());
            }

            config.accounts.push(Account {
                name: account,
                secret,
                issuer: issuer.unwrap_or("host".to_string()),
            });
            let path = Config::get_path()?;
            config.save_to_file(&path)?;

            println!("Account loaded")
        }
        Command::Import { file } => {
            let qr = qrcode::parse_qr_from_image(&file)?;
            config.accounts.push(qr);
            let path = Config::get_path()?;
            config.save_to_file(&path)?;

            println!("Account imported")
        }
    }

    Ok(())
}
