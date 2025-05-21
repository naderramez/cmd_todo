use clap::Parser;
use cli::{CliArgs, TodoAction};

mod cli;
mod export;
mod storage;
mod todo;

pub fn run() {
    let args = CliArgs::parse();
    match args.action {
        TodoAction::Create { content, category } => {
            cli::add_todo(&category, content);
        }
        TodoAction::List { category } => {
            cli::list_todos(&category).unwrap();
        }
        TodoAction::Export { category, path } => {
            cli::export_todos(&category, &path);
        }
        TodoAction::ExportAll { path } => {
            cli::export_all_todos(&path);
        }
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
