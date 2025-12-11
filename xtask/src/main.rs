use clap::{Parser, Subcommand};

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

fn main() {
    let args = Args::parse();

    match args.command {
        Command::Seed => {
            unimplemented!()
        }
    }
}
