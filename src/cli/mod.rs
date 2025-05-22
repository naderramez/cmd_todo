pub use args::{CliArgs, TodoAction};
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEventKind},
    execute, queue,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    io::{Result, Stdout, Write},
    path::PathBuf,
};

use crate::{
    export::export_data_to_file,
    todo::{self, Indexable, Todo, get_all_categories},
};

mod args;

pub fn add_todo(namespace: &str, content: String) {
    let todo = Todo::new(content);
    todo::add_todo(namespace, todo)
}

pub fn list_todos(namespace: &str) -> Result<()> {
    let mut todos = todo::get_todos(namespace);

    if todos.len() == 0 {
        eprintln!("Category not found!");
        return Ok(());
    }

    let mut selected = 0;

    let mut stdout = std::io::stdout();

    terminal::enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen, cursor::Hide)?;

    pub fn print_todo_list(mut stdout: &Stdout, todos: &Vec<Todo>, selected: usize) {
        // Clear and move to top-left
        queue!(stdout, cursor::MoveTo(0, 0), Clear(ClearType::All)).unwrap();

        stdout.flush().unwrap();

        for (i, todo) in todos.iter().enumerate() {
            let todo_line = format!(
                "{} - {}",
                todo.get_content(),
                todo.get_completion_status_text()
            );
            if i == selected {
                queue!(
                    stdout,
                    SetForegroundColor(Color::Yellow),
                    Print(format!("> {}\n", todo_line)),
                    ResetColor
                )
                .unwrap();
            } else {
                queue!(stdout, Print(format!("  {}\n", todo_line))).unwrap();
            }
            stdout.flush().unwrap();
        }

        println!("\n \n Available actions: [r]: remove  [s]: status toggle  [q]: quit");

        stdout.flush().unwrap(); // Ensure everything is drawn
    }

    print_todo_list(&stdout, &todos, selected);

    loop {
        // Key handling
        if let Event::Key(key_event) = event::read()? {
            if key_event.kind == KeyEventKind::Press {
                match key_event.code {
                    KeyCode::Up => {
                        if selected > 0 {
                            selected -= 1;
                        }
                        print_todo_list(&stdout, &todos, selected);
                    }
                    KeyCode::Down => {
                        if selected < todos.len() - 1 {
                            selected += 1;
                        }
                        print_todo_list(&stdout, &todos, selected);
                    }
                    KeyCode::Char('r') => {
                        if let Some(selected_todo) = todos.get(selected) {
                            todo::remove_todo(namespace, selected_todo.id());
                            todos = todo::get_todos(namespace);

                            if selected == todos.len() {
                                selected -= 1;
                            }
                            print_todo_list(&stdout, &todos, selected);

                            if todos.len() == 0 {
                                break;
                            }
                        }
                    }
                    KeyCode::Char('s') => {
                        if let Some(selected_todo) = todos.get(selected) {
                            todo::toggle_done_status(namespace, selected_todo.id());
                            todos = todo::get_todos(namespace);
                            print_todo_list(&stdout, &todos, selected);
                        }
                    }
                    KeyCode::Char('q') => break,
                    _ => {}
                }
            }
        }
    }

    // Cleanup
    execute!(stdout, LeaveAlternateScreen, cursor::Show)?;
    terminal::disable_raw_mode()?;
    Ok(())
}

pub fn export_todos(namespace: &str, export_path: &str) {
    let todos = todo::get_todos(namespace);
    let contents = todos
        .iter()
        .map(|todo| todo.get_content().to_string())
        .collect::<Vec<String>>()
        .join("\n");

    let path: PathBuf = export_path.into();
    export_data_to_file(contents, path);
}

pub fn export_all_todos(export_folder_path: &str) {
    let categories = get_all_categories();

    for category in categories {
        let mut file_path = PathBuf::from(export_folder_path);
        file_path = file_path.join(format!("{}.txt", category.clone()));
        export_todos(&category, file_path.to_str().unwrap());
    }
}
