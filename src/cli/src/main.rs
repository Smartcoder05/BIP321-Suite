mod commands;
use clap::{Parser, Subcommand};
use commands::build::BuildOptions;

#[derive(Parser)]
#[command(name = "bip321")]
#[command(about = "A BitcoinURI parser and builder")]
struct Cli {
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
enum Commands {
    Parse {
        uri: String
    },
    Validate {
        uri: String
    },
    Build {
        #[arg(long)]
        address: Option<String>,
        #[arg(long)]
        amount: Option<String>,
        #[arg(long)]
        label: Option<String>,
        #[arg(long)]
        message: Option<String>,
        #[arg(long)]
        pop: Option<String>,
        #[arg(long)]
        sp: Option<String>,
        #[arg(long)]
        lnd: Option<String>,
        #[arg(long)]
        lno: Option<String>,

    },
    Pop {
        uri: String,
        method: String,
        proof: String
    },
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Validate { uri } => commands::validate::run(uri),
        Commands::Parse { uri } => commands::parse::run(uri),
        Commands::Build { address, amount, 
            label, message, pop, lnd, lno, sp }
                    => commands::build::run(BuildOptions {address, amount, label, message, pop, lnd, lno, sp}),
        Commands::Pop { uri, method, proof } 
            => commands::pop::run(uri, method, proof),
    }
}