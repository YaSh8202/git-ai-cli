use clap::{command, Parser, Subcommand, };



#[derive(Parser)]
#[command(name="git-ai")]
#[command(about="AI Powered Cli tool for git commits")]
struct Cli{
    #[command(subcommand)]
    command: Commands
}


#[derive(Subcommand)]
enum Commands{
    Generate{
        #[arg(long)]
        staged: bool
    },
    Explain{
        #[arg(long)]
        staged: bool,
    }
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match  cli.command {
        Commands::Generate { staged } => {
            println!("Generate command");
            println!("Staged: {}", staged);
        }
        Commands::Explain { staged } => {
            println!("Explain command");
            println!("Staged: {}", staged);
        }
    }
}
