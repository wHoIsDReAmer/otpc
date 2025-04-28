mod commands;
mod config;
mod otp;
mod qrcode;

use clap::Parser as _;
use commands::Command;
use anyhow::Result;

fn main() -> Result<()> {
    let config = config::get_config()?;

    let command = Command::parse();
    
    // match command {
    //     Command::List => {
    // }

    println!("{}", config.accounts.len());

    Ok(())
}
