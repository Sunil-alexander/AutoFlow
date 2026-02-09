use clap::{Parser, Subcommand};

mod tasks;

#[derive(Parser)]
#[command(name = "AutoFlow")]
#[command(about = "Personal Automation CLI Tool")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init {
        project_type: String,
    },
    Organize {
        folder: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { project_type } => {
            tasks::init_project(&project_type);
        }
        Commands::Organize { folder } => {
            tasks::organize_folder(&folder);
        }
    }
} // <-- Make sure this closing brace exists
