mod commands;
mod config;
mod otp;

use clap::Parser as _;
use commands::Command;

fn main() {
    let command = Command::parse();
}
