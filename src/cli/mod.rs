pub use args::{CliArgs, TodoAction};
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEventKind},
    execute, queue,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{Result, Write};

use crate::todo::{self, Indexable, Todo};

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

    loop {
        // Clear and move to top-left
        execute!(stdout, cursor::MoveTo(0, 0), Clear(ClearType::All))?;

        stdout.flush()?;

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
                )?;
            } else {
                queue!(stdout, Print(format!("  {}\n", todo_line)))?;
            }
            stdout.flush()?
        }

        println!("\n \n Available actions: [r]: remove  [s]: status toggle  [q]: quit");

        stdout.flush()?; // Ensure everything is drawn

        // Key handling
        if let Event::Key(key_event) = event::read()? {
            if key_event.kind == KeyEventKind::Press {
                match key_event.code {
                    KeyCode::Up => {
                        if selected > 0 {
                            selected -= 1;
                        }
                    }
                    KeyCode::Down => {
                        if selected < todos.len() - 1 {
                            selected += 1;
                        }
                    }
                    KeyCode::Char('r') => {
                        if let Some(selected_todo) = todos.get(selected) {
                            todo::remove_todo(namespace, selected_todo.id());
                            todos = todo::get_todos(namespace);

                            if selected == todos.len() {
                                selected -= 1;
                            }

                            if todos.len() == 0 {
                                break;
                            }
                        }
                    }
                    KeyCode::Char('s') => {
                        if let Some(selected_todo) = todos.get(selected) {
                            todo::toggle_done_status(namespace, selected_todo.id());
                            todos = todo::get_todos(namespace);
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
