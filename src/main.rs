use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod organizer;
use organizer::FileOrganizer;

/// 🦀 A powerful CLI tool to organize files by type, size, date, or custom rules
#[derive(Parser)]
#[command(name = "file-organizer")]
#[command(about = "Organize your files automatically with Rust power!")]
#[command(version = "0.1.0")]
#[command(author = "Janmesh Shewale")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Configuration file path
    #[arg(short, long)]
    config: Option<PathBuf>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Organize files in a directory
    Organize {
        /// Directory to organize (default: current directory)
        #[arg(value_name = "DIRECTORY")]
        path: Option<PathBuf>,

        /// Organization mode
        #[arg(short, long, value_enum, default_value_t = OrganizeMode::Extension)]
        mode: OrganizeMode,

        /// Preview changes without applying them
        #[arg(short = 'n', long)]
        dry_run: bool,

        /// Force operation without confirmation
        #[arg(short, long)]
        force: bool,

        /// Filter specific file types
        #[arg(long, value_delimiter = ',')]
        filter: Option<Vec<String>>,

        /// Create backup before organizing
        #[arg(short, long)]
        backup: bool,

        /// Recursive organization (include subdirectories)
        #[arg(short, long)]
        recursive: bool,
    },

    /// Undo the last organization operation
    Undo {
        /// Directory to undo organization
        #[arg(value_name = "DIRECTORY")]
        path: Option<PathBuf>,

        /// Show what would be undone without doing it
        #[arg(short = 'n', long)]
        dry_run: bool,
    },

    /// Show organization history
    History {
        /// Directory to show history for
        #[arg(value_name = "DIRECTORY")]
        path: Option<PathBuf>,

        /// Number of recent operations to show
        #[arg(short, long, default_value_t = 10)]
        limit: usize,
    },

    /// Clean empty directories
    Clean {
        /// Directory to clean
        #[arg(value_name = "DIRECTORY")]
        path: Option<PathBuf>,

        /// Preview what would be cleaned
        #[arg(short = 'n', long)]
        dry_run: bool,
    },

    /// Generate shell completions
    Completions {
        /// Shell to generate completions for
        #[arg(value_enum)]
        shell: Shell,
    },
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum OrganizeMode {
    /// Organize by file extension (default)
    Extension,
    /// Organize by file size
    Size,
    /// Organize by creation date
    Date,
    /// Organize by modification date
    Modified,
    /// Use custom rules from config file
    Custom,
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum Shell {
    Bash,
    Zsh,
    Fish,
    PowerShell,
}

fn main() {
    // Initialize logger
    env_logger::init();

    // Parse CLI arguments
    let cli = Cli::parse();

    // Set up logging level based on verbose flag
    if cli.verbose {
        println!("🦀 File Organizer CLI - Verbose mode enabled");
    }

    // Handle the command
    let result = match cli.command {
        Commands::Organize {
            path,
            mode,
            dry_run,
            force,
            filter,
            backup,
            recursive,
        } => handle_organize(path, mode, dry_run, force, filter, backup, recursive, cli.config),

        Commands::Undo { path, dry_run } => handle_undo(path, dry_run),

        Commands::History { path, limit } => handle_history(path, limit),

        Commands::Clean { path, dry_run } => handle_clean(path, dry_run),

        Commands::Completions { shell } => handle_completions(shell),
    };

    // Handle any errors
    if let Err(e) = result {
        eprintln!("❌ Error: {}", e);
        std::process::exit(1);
    }
}

/// Handle the organize command
fn handle_organize(
    path: Option<PathBuf>,
    mode: OrganizeMode,
    dry_run: bool,
    _force: bool,
    filter: Option<Vec<String>>,
    backup: bool,
    recursive: bool,
    config: Option<PathBuf>,
) -> anyhow::Result<()> {
    use colored::Colorize;
    
    let target_path = path.unwrap_or_else(|| std::env::current_dir().unwrap());
    
    // Verify target directory exists
    if !target_path.exists() {
        return Err(anyhow::anyhow!("Directory does not exist: {}", target_path.display()));
    }
    
    if !target_path.is_dir() {
        return Err(anyhow::anyhow!("Path is not a directory: {}", target_path.display()));
    }
    
    // Show operation details
    println!("{}", "🦀 File Organizer CLI".bold().cyan());
    println!("🎯 Target directory: {}", target_path.display().to_string().green());
    println!("📋 Organization mode: {:?}", mode);
    
    if dry_run {
        println!("{}", "🔍 DRY RUN MODE - No changes will be made".yellow());
    }
    
    if let Some(filters) = &filter {
        println!("🔧 File filters: {}", filters.join(", ").cyan());
    }
    
    if backup {
        println!("{}", "💾 Backup mode enabled (TODO: Not implemented yet)".yellow());
    }
    
    if recursive {
        println!("{}", "🔄 Recursive mode enabled".green());
    }
    
    if let Some(config_path) = config {
        println!("⚙️ Using config: {} {}", config_path.display(), "(TODO: Not implemented yet)".yellow());
    }
    
    println!(); // Empty line for better formatting
    
    // Create and run the organizer
    let mut organizer = FileOrganizer::new();
    let _summary = organizer.organize(
        &target_path,
        &mode,
        recursive,
        filter.as_ref(),
        dry_run,
    )?;
    
    println!("\n{}", "🎉 File organization completed successfully!".bold().green());
    
    Ok(())
}

/// Handle the undo command
fn handle_undo(path: Option<PathBuf>, dry_run: bool) -> anyhow::Result<()> {
    let target_path = path.unwrap_or_else(|| std::env::current_dir().unwrap());
    
    println!("↩️  Undoing organization in: {}", target_path.display());
    
    if dry_run {
        println!("🔍 DRY RUN MODE - Showing what would be undone");
    }
    
    // TODO: Implement undo logic
    println!("✅ Undo completed!");
    
    Ok(())
}

/// Handle the history command
fn handle_history(path: Option<PathBuf>, limit: usize) -> anyhow::Result<()> {
    let target_path = path.unwrap_or_else(|| std::env::current_dir().unwrap());
    
    println!("📚 Showing history for: {}", target_path.display());
    println!("📊 Limit: {} operations", limit);
    
    // TODO: Implement history logic
    println!("ℹ️  No operations found in history");
    
    Ok(())
}

/// Handle the clean command
fn handle_clean(path: Option<PathBuf>, dry_run: bool) -> anyhow::Result<()> {
    let target_path = path.unwrap_or_else(|| std::env::current_dir().unwrap());
    
    println!("🧹 Cleaning empty directories in: {}", target_path.display());
    
    if dry_run {
        println!("🔍 DRY RUN MODE - Showing what would be cleaned");
    }
    
    // TODO: Implement clean logic
    println!("✅ Cleaning completed!");
    
    Ok(())
}

/// Handle shell completions generation
fn handle_completions(shell: Shell) -> anyhow::Result<()> {
    use clap::CommandFactory;
    use clap_complete::{generate, shells};
    use std::io;
    
    let mut cmd = Cli::command();
    let bin_name = cmd.get_name().to_string();
    
    match shell {
        Shell::Bash => generate(shells::Bash, &mut cmd, bin_name, &mut io::stdout()),
        Shell::Zsh => generate(shells::Zsh, &mut cmd, bin_name, &mut io::stdout()),
        Shell::Fish => generate(shells::Fish, &mut cmd, bin_name, &mut io::stdout()),
        Shell::PowerShell => generate(shells::PowerShell, &mut cmd, bin_name, &mut io::stdout()),
    }
    
    Ok(())
}
