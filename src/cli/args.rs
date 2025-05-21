use clap::{Parser, Subcommand};

#[derive(Debug, Clone, Subcommand)]
pub enum TodoAction {
    /// Create new todo
    Create {
        /// Category of the todo
        #[arg(index = 1)]
        category: String,
        /// Todo content
        #[arg(index = 2)]
        content: String,
    },
    /// List todos
    List {
        /// Category of the todo
        #[arg()]
        category: String,
    },
    /// Exports all todos of a category into a file that you specify
    Export {
        /// Category of the todo
        #[arg(index = 1)]
        category: String,
        /// Path for exporting todos
        #[arg(index = 2)]
        path: String,
    },
    /// Exports all todos of all categories into a folder that you specify
    ExportAll {
        /// Path for exporting todos
        #[arg()]
        path: String,
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
