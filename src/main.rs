use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

/// A minimal dataset packaging tool
#[derive(Debug, Parser)]
#[command(name = "seamless")]
#[command(about = "A minimal dataset packaging tool.", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Pull repos
    #[command(arg_required_else_help = true)]
    Pull {
        /// Dataset path
        path: String,
        /// Destination path
        destination: Option<PathBuf>,
    },
    /// Run SQL queries over a dataset
    #[command(arg_required_else_help = true)]
    Query {
        /// Dataset path
        path: String,
    },
    /// Validates a dataset exists
    #[command(arg_required_else_help = true)]
    Validate {
        /// Dataset path
        #[arg(required = true)]
        path: Vec<PathBuf>,
    },
}

#[derive(Debug, Args)]
struct StashPushArgs {
    #[arg(short, long)]
    message: Option<String>,
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Pull { path, destination } => {
            println!("Pulling {path} into {destination:?}");
        }
        Commands::Query { path } => {
            println!("Querying {path}");
        }
        Commands::Validate { path } => {
            println!("Validating {path:?}");
        }
    }
}
