
use clap::{Parser, Subcommand};
use anyhow::Result;
use tracing::{info, error};
use wasmrunner_core::config::Config;

mod commands;
mod utils;

#[derive(Parser)]
#[command(name = "wasmrunner")]
#[command(about = "A containerless WASM application runner")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    
    #[arg(long, global = true)]
    config: Option<String>,
    
    #[arg(long, global = true)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a WASM application
    Run {
        /// Image name with optional tag (e.g., hello-world:latest)
        image: String,
        
        /// Memory limit in MB
        #[arg(long, default_value = "128")]
        memory: u64,
        
        /// CPU limit (percentage)
        #[arg(long, default_value = "100")]
        cpu: u32,
        
        /// Environment variables (KEY=VALUE)
        #[arg(long, short)]
        env: Vec<String>,
        
        /// Network mode: none, host
        #[arg(long, default_value = "none")]
        network: String,
        
        /// Run in detached mode
        #[arg(long, short)]
        detach: bool,
        
        /// Container name
        #[arg(long)]
        name: Option<String>,
        
        /// Arguments to pass to the WASM app
        args: Vec<String>,
    },
    
    /// Build a WASM application
    Build {
        /// Tag for the built image
        #[arg(long, short)]
        tag: String,
        
        /// Build context directory
        #[arg(default_value = ".")]
        context: String,
        
        /// Dockerfile path
        #[arg(long, short, default_value = "Dockerfile.wasm")]
        file: String,
    },
    
    /// Push an image to registry
    Push {
        /// Image name with tag
        image: String,
    },
    
    /// Pull an image from registry
    Pull {
        /// Image name with tag
        image: String,
    },
    
    /// List running containers
    List {
        /// Show all containers (including stopped)
        #[arg(long, short)]
        all: bool,
    },
    
    /// Show container logs
    Logs {
        /// Container ID or name
        container: String,
        
        /// Follow log output
        #[arg(long, short)]
        follow: bool,
        
        /// Number of lines to show
        #[arg(long, default_value = "100")]
        tail: u32,
    },
    
    /// Stop a running container
    Stop {
        /// Container ID or name
        container: String,
    },
    
    /// Remove a container
    Remove {
        /// Container ID or name
        container: String,
        
        /// Force removal
        #[arg(long, short)]
        force: bool,
    },
    
    /// Search for apps in the app store
    Search {
        /// Search term
        term: String,
        
        /// Category filter
        #[arg(long)]
        category: Option<String>,
        
        /// Show only verified apps
        #[arg(long)]
        verified: bool,
    },
    
    /// Install an app from the store
    Install {
        /// App name or slug
        name: String,
        
        /// Specific version
        #[arg(long)]
        version: Option<String>,
    },
    
    /// Login to WasmRunner app store
    Login {
        /// Email address
        #[arg(long)]
        email: Option<String>,
    },
    
    /// Register new account
    Register {
        /// Email address
        #[arg(long)]
        email: Option<String>,
        
        /// Username
        #[arg(long)]
        username: Option<String>,
    },
    
    /// Publish app to store
    Publish {
        /// Image name with tag
        image: String,
        
        /// Manifest file path
        #[arg(long)]
        manifest: Option<String>,
        
        /// App description
        #[arg(long)]
        description: Option<String>,
        
        /// App category
        #[arg(long)]
        category: Option<String>,
        
        /// Tags
        #[arg(long)]
        tag: Vec<String>,
    },
    
    /// Manage favorites
    Favorite {
        /// App name to favorite/unfavorite
        app: String,
        
        /// Remove from favorites
        #[arg(long)]
        remove: bool,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // Initialize logging
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(if cli.verbose {
            tracing::Level::DEBUG
        } else {
            tracing::Level::INFO
        })
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;
    
    // Load configuration
    let config = Config::load(cli.config.as_deref())?;
    info!("WasmRunner starting with config: {:?}", config);
    
    // Execute command
    match cli.command {
        Commands::Run { 
            image, memory, cpu, env, network, detach, name, args 
        } => {
            commands::run::execute(image, memory, cpu, env, network, detach, name, args).await
        },
        Commands::Build { tag, context, file } => {
            commands::build::execute(tag, context, file).await
        },
        Commands::Push { image } => {
            commands::push::execute(image).await
        },
        Commands::Pull { image } => {
            commands::pull::execute(image).await
        },
        Commands::List { all } => {
            commands::list::execute(all).await
        },
        Commands::Logs { container, follow, tail } => {
            commands::logs::execute(container, follow, tail).await
        },
        Commands::Stop { container } => {
            commands::stop::execute(container).await
        },
        Commands::Remove { container, force } => {
            commands::remove::execute(container, force).await
        },
        Commands::Search { term, category, verified } => {
            commands::search::execute(term, category, verified).await
        },
        Commands::Install { name, version } => {
            commands::install::execute(name, version).await
        },
        Commands::Login { email } => {
            commands::login::execute(email).await
        },
        Commands::Register { email, username } => {
            commands::register::execute(email, username).await
        },
        Commands::Publish { image, manifest, description, category, tag } => {
            commands::publish::execute(image, manifest, description, category, tag).await
        },
        Commands::Favorite { app, remove } => {
            commands::favorite::execute(app, remove).await
        },
    }
}
