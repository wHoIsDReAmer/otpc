mod commands;
mod config;

use clap::Parser as _;
use commands::Command;

fn main() {
    let command = Command::parse();
}
