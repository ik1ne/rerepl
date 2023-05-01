use std::io::prelude::*;
use std::io::Stdout;

use crossterm::cursor::{MoveLeft, MoveRight, MoveToColumn, MoveToNextLine};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::terminal::{Clear, ClearType};
use crossterm::{cursor, event, execute, terminal};

use crate::rerepl::parent::SharedData;

pub(crate) fn start_handling_terminal_input(shared_data: &SharedData) -> Result<(), anyhow::Error> {
    terminal::enable_raw_mode().unwrap();

    let mut stdout = shared_data.parent_stdout.lock().unwrap();
    write!(stdout, "{}", shared_data.prompt)?;
    stdout.flush().unwrap();

    Ok(())
}

pub(crate) fn handle_child_stdout(shared: &mut SharedData, line: String) {
    let mut stdout = shared.parent_stdout.lock().unwrap();
    // clear the current line ("prompt> user_input").
    let _ = clear_current_line(&mut stdout);

    // print out the line from the child process.
    writeln!(stdout, "{}", line).unwrap();

    // move cursor accordingly.
    let rows = count_rows(&line);
    execute!(stdout, MoveToNextLine(rows), MoveToColumn(0)).unwrap();

    // restore prompt and buffer.
    let _ = restore_prompt_and_buffer(&mut stdout, shared);
}

fn count_rows(line: &str) -> u16 {
    let mut row = 1u16;
    for c in line.chars() {
        if c == '\n' {
            row = row.saturating_add(1);
        }
    }

    row
}

pub(crate) fn handle_parent_stdin(
    shared: &mut SharedData,
    child_stdin: &mut (impl Write + Sized),
) -> bool {
    if let Event::Key(KeyEvent {
        code, modifiers, ..
    }) = event::read().unwrap()
    {
        let mut stdout = shared.parent_stdout.lock().unwrap();
        let mut buffer = shared.parent_stdin_buffer.lock().unwrap();
        match code {
            KeyCode::Char('c') if modifiers.contains(KeyModifiers::CONTROL) => {
                // clear the terminal
                clear_current_line(&mut stdout).unwrap();
                buffer.clear();
                shared.cursor_idx = 0;
                restore_prompt_and_buffer(&mut stdout, shared).unwrap();
            }
            KeyCode::Char('d') if modifiers.contains(KeyModifiers::CONTROL) => {
                // stop the child loop and program
                writeln!(child_stdin, "exit").unwrap();
                writeln!(stdout).unwrap();
                execute!(stdout, cursor::MoveToColumn(0), cursor::MoveToNextLine(1)).unwrap();
                stdout.flush().unwrap();
                return false;
            }
            KeyCode::Char(c) => {
                buffer.insert(shared.cursor_idx, c);
                shared.cursor_idx += 1;
                execute!(
                    stdout,
                    cursor::MoveToColumn(0),
                    Clear(ClearType::CurrentLine)
                )
                .unwrap();
                write!(stdout, "{}{}", shared.prompt, buffer).unwrap();
                stdout.flush().unwrap();
                execute!(
                    stdout,
                    cursor::MoveToColumn((shared.cursor_idx + shared.prompt.len()) as u16)
                )
                .unwrap();
            }
            KeyCode::Backspace => {
                if shared.cursor_idx > 0 {
                    buffer.remove(shared.cursor_idx - 1);
                    shared.cursor_idx -= 1;
                    execute!(
                        stdout,
                        MoveLeft(1),
                        cursor::MoveToColumn(0),
                        Clear(ClearType::CurrentLine)
                    )
                    .unwrap();
                    write!(stdout, "{}{}", shared.prompt, buffer).unwrap();
                    stdout.flush().unwrap();
                    execute!(
                        stdout,
                        cursor::MoveToColumn((shared.cursor_idx + shared.prompt.len()) as u16)
                    )
                    .unwrap();
                }
            }
            KeyCode::Delete => {
                if shared.cursor_idx < buffer.len() {
                    buffer.remove(shared.cursor_idx);
                    execute!(
                        stdout,
                        cursor::MoveToColumn(0),
                        Clear(ClearType::CurrentLine)
                    )
                    .unwrap();
                    write!(stdout, "{}{}", shared.prompt, buffer).unwrap();
                    stdout.flush().unwrap();
                    execute!(
                        stdout,
                        cursor::MoveToColumn((shared.cursor_idx + shared.prompt.len()) as u16)
                    )
                    .unwrap();
                }
            }
            KeyCode::Left => {
                if shared.cursor_idx > 0 {
                    shared.cursor_idx -= 1;
                    execute!(stdout, MoveLeft(1)).unwrap();
                }
            }
            KeyCode::Right => {
                if shared.cursor_idx < buffer.len() {
                    shared.cursor_idx += 1;
                    execute!(stdout, MoveRight(1)).unwrap();
                }
            }
            KeyCode::Enter => {
                // Send the input buffer to the child process when the user presses Enter
                writeln!(child_stdin, "{}", buffer).unwrap();
                child_stdin.flush().unwrap();

                // Clear the buffer and reset the cursor index
                buffer.clear();
                shared.cursor_idx = 0;

                // move the cursor to next line and print the prompt again
                writeln!(stdout).unwrap();
                stdout.flush().unwrap();
                execute!(stdout, cursor::MoveToNextLine(2), cursor::MoveToColumn(0)).unwrap();
                write!(stdout, "{}", shared.prompt).unwrap();
                stdout.flush().unwrap();
            }
            _ => {}
        }
    }

    true
}

pub fn stop_handling_terminal_input() -> Result<(), anyhow::Error> {
    terminal::disable_raw_mode().unwrap();

    Ok(())
}

fn clear_current_line(stdout: &mut Stdout) -> Result<(), anyhow::Error> {
    execute!(
        stdout,
        cursor::MoveToColumn(0),
        Clear(ClearType::CurrentLine)
    )
    .map_err(|e| e.into())
}

fn restore_prompt_and_buffer(
    stdout: &mut Stdout,
    shared: &SharedData,
) -> Result<(), anyhow::Error> {
    let stdin_buffer = shared.parent_stdin_buffer.lock().unwrap();

    write!(stdout, "{}{}", shared.prompt, stdin_buffer)?;

    execute!(
        stdout,
        cursor::MoveToColumn((shared.cursor_idx + shared.prompt.len()) as u16)
    )
    .unwrap();

    stdout.flush().unwrap();

    Ok(())
}
