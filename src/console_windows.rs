use std::io::{Write, stdout};
use std::time::Duration;

use crossterm::terminal;
use crossterm::event;
use crossterm::queue;
use crossterm::style;

use super::terminal::TerminalEmulator;

pub struct Console {
    next_char: Option<u8>,
    terminal: Box<dyn TerminalEmulator>
}

impl Console {
    pub fn new(terminal: Box<dyn TerminalEmulator>) -> Console {
        terminal::enable_raw_mode().unwrap();

        Console {
            next_char: None,
            terminal: terminal,
        }
    }
}

impl Console {
    pub fn status(&mut self) -> bool {
        match self.next_char {
            Some(_) => true,
            None => {
                loop {
                    if event::poll(Duration::from_nanos(100)).unwrap() {
                        let event = event::read().unwrap();
                        let some_ch = event_to_char(event);
                        if let Some(ch) = some_ch {
                            self.next_char = Some(ch);
                            break true
                        }
                        // The event is not a valid char, ignore and retry
                    } else {
                        break false
                    }
                }
            }
        }
    }

    pub fn read(&mut self) -> u8 {
        match self.next_char {
            Some(ch) => {
                self.next_char = None;
                ch
            },
            None => {
                loop {
                    let event = event::read().unwrap();
                    let some_ch = event_to_char(event);
                    if let Some(ch) = some_ch {
                        break ch;
                    }
                    // The event is not a valid char, ignore and retry
                }
            }
        }
    }

    pub fn put(&mut self, ch: u8) {
        if let Some(sequence) = self.terminal.translate(ch) {
            queue!(stdout(), style::Print(sequence)).unwrap();
            stdout().flush().unwrap();
        }
    }
}

impl Drop for Console {
    fn drop(&mut self) {
        terminal::disable_raw_mode().unwrap();
    }
}

fn event_to_char(event: event::Event) -> Option<u8> {
    let a = match event {
        event::Event::Key(k) => match k.code {
            event::KeyCode::Char(c) => {
                if k.modifiers == event::KeyModifiers::NONE ||
                        k.modifiers == event::KeyModifiers::SHIFT {
                    if ' ' <= c && c <= '~' {
                        // Valid ASCII, not control, char
                        Some(c as u8)
                    } else {
                        None
                    }
                } else if k.modifiers == event::KeyModifiers::CONTROL {
                    if '`' <= c && c <= '~' {
                        // Valid control range
                        Some(c as u8 - '`' as u8)
                    } else {
                        None
                    }
                } else {
                    None
                }
            },
            event::KeyCode::Backspace => Some(127),
            event::KeyCode::Enter => Some(13),
            event::KeyCode::Left =>  Some(8),
            event::KeyCode::Right => Some(12),
            event::KeyCode::Up => Some(11),
            event::KeyCode::Down => Some(10),
            event::KeyCode::Home => Some(30),
            event::KeyCode::Tab => Some(9),
            event::KeyCode::Esc => Some(27),
            _ => None, // We ignore: End, PageUp, PageDown, BackTab, Delete, Insert, F(n)
        },
        _ => None, // Not a keyboard event, ignore.
    };

    a
}