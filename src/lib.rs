use clap::Parser;
use cli::{CliArgs, CliManager, TodoAction};

mod cli;
mod storage;
mod todo;

pub fn run() {
    let args = CliArgs::parse();
    let mut cli_manager: Option<CliManager> = None;
    match args.action {
        TodoAction::Create { content, category } => {
            if cli_manager.is_none() {
                cli_manager = Some(CliManager::new(category));
            }
            cli_manager.unwrap().add_todo(content);
        }
        TodoAction::List { category } => {
            if cli_manager.is_none() {
                cli_manager = Some(CliManager::new(category));
            }
            cli_manager.unwrap().list_todos().unwrap();
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
