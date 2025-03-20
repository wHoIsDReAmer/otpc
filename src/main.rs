use clap::Parser as _;
use commands::Command;

mod commands;

fn main() {
    let command = Command::parse();
}
