use std::io::{stdout, Write};

use crossterm::event::KeyModifiers;
use crossterm::{
    cursor::{self, MoveLeft, MoveRight},
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};

const PROMPT: &str = "> ";

fn main() {
    let mut stdout = stdout();
    let mut buffer = String::new();
    let mut cursor_idx = 0;

    enable_raw_mode().unwrap();

    // Print the prompt
    print!("{}", PROMPT);
    stdout.flush().unwrap();

    loop {
        // Read user input
        if let Event::Key(KeyEvent {
            code, modifiers, ..
        }) = event::read().unwrap()
        {
            match code {
                KeyCode::Char(c) => {
                    if c == 'b' && modifiers.contains(KeyModifiers::CONTROL) {
                        // Backup buffer when pressing Ctrl+B
                        execute!(
                            stdout,
                            cursor::MoveToColumn(0),
                            Clear(ClearType::CurrentLine)
                        )
                        .unwrap();
                        println!("{}", buffer);
                        execute!(stdout, cursor::MoveToColumn(0), cursor::MoveToNextLine(1))
                            .unwrap();
                        print!("{}{}", PROMPT, buffer);
                        stdout.flush().unwrap();
                        execute!(
                            stdout,
                            cursor::MoveToColumn((cursor_idx + PROMPT.len()) as u16)
                        )
                        .unwrap();
                    // +2 to account for the "> " prompt
                    } else {
                        // Insert character at the cursor position
                        buffer.insert(cursor_idx, c);
                        cursor_idx += 1;
                        execute!(
                            stdout,
                            cursor::MoveToColumn(0),
                            Clear(ClearType::CurrentLine)
                        )
                        .unwrap();
                        print!("> {}", buffer); // Include the prompt before the buffer
                        stdout.flush().unwrap();
                        execute!(
                            stdout,
                            cursor::MoveToColumn((cursor_idx + PROMPT.len()) as u16)
                        )
                        .unwrap();
                    }
                }
                KeyCode::Backspace => {
                    if cursor_idx > 0 {
                        buffer.remove(cursor_idx - 1);
                        cursor_idx -= 1;
                        execute!(
                            stdout,
                            MoveLeft(1),
                            cursor::MoveToColumn(0),
                            Clear(ClearType::CurrentLine)
                        )
                        .unwrap();
                        print!("> {}", buffer); // Include the prompt before the buffer
                        stdout.flush().unwrap();
                        execute!(
                            stdout,
                            cursor::MoveToColumn((cursor_idx + PROMPT.len()) as u16)
                        )
                        .unwrap();
                    }
                }
                KeyCode::Delete => {
                    if cursor_idx < buffer.len() {
                        buffer.remove(cursor_idx);
                        execute!(
                            stdout,
                            cursor::MoveToColumn(0),
                            Clear(ClearType::CurrentLine)
                        )
                        .unwrap();
                        print!("> {}", buffer); // Include the prompt before the buffer
                        stdout.flush().unwrap();
                        execute!(
                            stdout,
                            cursor::MoveToColumn((cursor_idx + PROMPT.len()) as u16)
                        )
                        .unwrap();
                    }
                }
                KeyCode::Left => {
                    if cursor_idx > 0 {
                        cursor_idx -= 1;
                        execute!(stdout, MoveLeft(1)).unwrap();
                    }
                }
                KeyCode::Right => {
                    if cursor_idx < buffer.len() {
                        cursor_idx += 1;
                        execute!(stdout, MoveRight(1)).unwrap();
                    }
                }
                KeyCode::Enter => {
                    execute!(stdout, cursor::MoveToColumn(0), cursor::MoveToNextLine(1)).unwrap();
                    println!("Input: {}", buffer);
                    execute!(stdout, cursor::MoveToColumn(0), cursor::MoveToNextLine(1)).unwrap();
                    stdout.flush().unwrap();
                    break;
                }
                KeyCode::Esc => {
                    break;
                }
                _ => {}
            }
        }
    }

    disable_raw_mode().unwrap();
}
