mod commands;
mod config;
mod otp;
mod qrcode;

use clap::Parser as _;
use commands::Command;
use anyhow::Result;

use otp::{Otp, OtpType};

fn main() -> Result<()> {
    let mut config = config::get_config()?
        .lock()
        .map_err(|e| anyhow::anyhow!(e.to_string()))?;

    let command = Command::parse();
    match command {
        Command::List => {
            for account in config.accounts.iter() {
                println!("{}", account.name);
            }
        }
        Command::Code { account } => {
            let account = config.accounts.iter().find(|acc| acc.name == account)
                .ok_or(anyhow::anyhow!("Account not found"))?;

            let otp = Otp::new(account.secret.as_str(), 6, 30, OtpType::Totp);
            let code = otp.generate_code();
            println!("{}", code);
        }
        Command::Delete { account } => {
            config.accounts.retain(|acc| acc.name != account);
        }
        Command::Import { file } => {
            // let qr = qrcode::parse_qr_from_image(file)?;
            // println!("{}", qr);
        }
        _ => {}
    }

    Ok(())
}
