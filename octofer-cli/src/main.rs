//! # Octofer CLI
//! 
//! Command-line interface for creating and managing Octofer applications.

use clap::{Parser, Subcommand};
use anyhow::Result;

#[derive(Parser)]
#[command(name = "octofer")]
#[command(about = "A CLI for creating and managing Octofer GitHub Apps")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new Octofer application
    New {
        /// Name of the application
        name: String,
        /// Output directory
        #[arg(short, long, default_value = ".")]
        output: String,
    },
    /// Start development server
    Dev {
        /// Port to run on
        #[arg(short, long, default_value = "3000")]
        port: u16,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::New { name, output } => {
            println!("Creating new Octofer app: {} in {}", name, output);
            create_app(&name, &output).await?;
        }
        Commands::Dev { port } => {
            println!("Starting development server on port {}", port);
            start_dev_server(port).await?;
        }
    }

    Ok(())
}

async fn create_app(name: &str, output: &str) -> Result<()> {
    // Implementation for creating a new app scaffold
    println!("App '{}' created successfully in '{}'", name, output);
    Ok(())
}

async fn start_dev_server(port: u16) -> Result<()> {
    // Implementation for development server
    println!("Development server running on http://localhost:{}", port);
    Ok(())
}