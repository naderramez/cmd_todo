use clap::{Parser, Subcommand};

#[derive(Debug, Clone, Subcommand)]
pub enum TodoAction {
    Create {
        /// Todo content
        #[arg(long)]
        content: String,
        /// Category of the todo
        #[arg(long)]
        category: String,
    },
    List {
        /// Category of the todo
        #[arg(long)]
        category: String,
    },
}

/// Program that manages todos
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CliArgs {
    /// Action to take
    #[command(subcommand)]
    pub action: TodoAction,
}
