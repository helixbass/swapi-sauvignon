use clap::{Parser, Subcommand};

mod seed;

use seed::seed;

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// seed the DB
    Seed,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    match args.command {
        Command::Seed => {
            seed().await;
        }
    }
}
