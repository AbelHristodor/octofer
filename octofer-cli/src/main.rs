//! # Octofer CLI
//!
//! Command-line interface for scaffolding and managing Octofer GitHub Apps.

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "octofer")]
#[command(about = "A CLI for scaffolding GitHub Apps with Octofer")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new Octofer GitHub App project
    New {
        /// Name of the new project
        name: String,
        /// Directory to create the project in (defaults to current directory)
        #[arg(short, long)]
        path: Option<String>,
    },
    /// Development server commands
    Dev {
        /// Port to run the development server on
        #[arg(short, long, default_value_t = 3000)]
        port: u16,
        /// Host to bind the development server to
        #[arg(long, default_value = "127.0.0.1")]
        host: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::New { name, path } => {
            create_new_project(name, path.as_deref()).await?;
        }
        Commands::Dev { port, host } => {
            start_dev_server(host, *port).await?;
        }
    }

    Ok(())
}

/// Create a new Octofer project
async fn create_new_project(name: &str, path: Option<&str>) -> Result<()> {
    println!("Creating new Octofer project: {}", name);

    let project_path = path.unwrap_or(".");
    println!("Project will be created in: {}", project_path);

    // TODO: Implement project scaffolding
    println!("🚧 Project scaffolding is not yet implemented");
    println!("This will create a new Octofer GitHub App project with:");
    println!("  - Basic project structure");
    println!("  - Configuration templates");
    println!("  - Example event handlers");
    println!("  - Docker configuration");
    println!("  - GitHub Actions workflows");

    Ok(())
}

/// Start the development server
async fn start_dev_server(host: &str, port: u16) -> Result<()> {
    println!("🚧 Development server is not yet implemented");
    println!("This will start a development server with:");
    println!("  - Hot reloading");
    println!("  - Webhook tunneling");
    println!("  - Local GitHub App simulation");
    println!("  - Debug logging");
    println!();
    println!("Server would start on: http://{}:{}", host, port);

    Ok(())
}
