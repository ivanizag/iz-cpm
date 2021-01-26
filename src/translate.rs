/*
See:
    http://bitsavers.informatik.uni-stuttgart.de/pdf/kaypro/3318-A_Kaypro_Robbie_Users_Guide_Feb85.pdf
    http://ascii-table.com/ansi-escape-sequences-vt-100.php
    https://www.xfree86.org/current/ctlseqs.html
    http://skookumpete.com/KayproGraphics.htm
    https://docs.microsoft.com/en-us/windows/console/console-virtual-terminal-sequences


Control characters:
    Ring Bell                      07 -> 07
    Cursor left (non-destructive)  08 -> ESC[D
    Cursor Right                   12 -> ESC[C
    Cursor Down                    10 -> ESC[B
    Cursor Up                      11 -> ESC[A
    Erase to end of screen         23 -> ESC[0J
    Erase to end of line           24 -> ESC[0K
    Clear screen, home cursor      26 -> ESC[2J + ESC[H
    Home cursor                    30 -> ESC[H

ESCape Sequences
    Insert line                                 ESC,R   -> ESC[L (Windows only?)
    Delete line                                 ESC,E   -> ESC[M (Windows only?)
    Cursor address                              ESC,=,row+32,col+32 -> ESC[n;mH
    Reverse video start                         ESC,B,0 -> ESC[7m
    Reverse video stop                          ESC,C,0 -> ESC[27m
    Half intensity start                        ESC,B,1 -> ESC[2m ??
    Half intensity stop                         ESC,C,1 -> ESC[22m]
    Blinking start                              ESC,B,2 -> ESC[5m
    Blinking stop                               ESC,C,2 -> ESC[25m
    Underline start                             ESC,B,3 -> ESC[4m
    Underline stop                              ESC,C,3 -> ESC[24m
    Cursor on                                   ESC,B,4 -> ESC?25h
    Cursor off                                  ESC,C,4 -> ESC?25l
    Video mode on                               ESC,B,5 -> ??
    Video mode off                              ESC,C,5 -> ??
    Remember current cursor position            ESC,B,6 -> ESC[s (win) or ESC7 (xterm) 
    Return to last remembered cursor position   ESC,C,6 -> ESC[u (win) or ESC8 (xterm)
    Status line preservation on                 ESC,B,7 -> ??
    Status line preservation off                ESC,C,7 -> ??
*/

pub struct Adm3aToAnsi {
    buffer: [u8;4],
    buffer_len: usize
}

impl Adm3aToAnsi {
    pub fn new() -> Adm3aToAnsi {
        Adm3aToAnsi {
            buffer: [0,0,0,0],
            buffer_len: 0
        }
    }

    fn conversion(&mut self) -> Option<String> {
        match self.buffer[0] {
            // Control characters
            3 => Some("".to_string()),           // Invisible control-C
            8  => Some("\x1b[D".to_string()),    // Cursor left (non-destructive)
            // 10 => Some("\x1b[B".to_string()),    // Cursor Down
            11 => Some("\x1b[A".to_string()),    // Cursor Up
            12 => Some("\x1b[C".to_string()),    // Cursor Right
            23 => Some("\x1b[J".to_string()),    // Erase to end of screen
            24 => Some("\x1b[K".to_string()),    // Erase to end of line
            26 => Some("\x1b[2J\x1b[H".to_string()),   // Clear screen, home cursor
            30 => Some("\x1b[H".to_string()),    // Home cursor
            127 => Some("\x1b[D \x1b[D".to_string()), // Del: back+space+back
            // Escape sequences 
            27 => { // ESCAPE
                if self.buffer_len < 2 {
                    None // We need more buffer
                } else {
                    match self.buffer[1] as char {
                        'R' => Some("\x1b[L".to_string()), // Insert line
                        'E' => Some("\x1b[M".to_string()), // Delete line
                        'B' => {
                            if self.buffer_len < 3 {
                                None // We need more buffer
                            } else {
                                match self.buffer[2] as char {
                                    '0' => Some("\x1b[7m".to_string()),   // Reverse video start
                                    '1' => Some("\x1b[2m".to_string()),   // Half intensity start
                                    '2' => Some("\x1b[5m".to_string()),   // Blinking start
                                    '3' => Some("\x1b[4m".to_string()),   // Underline start
                                    '4' => Some("\x1b[?25h".to_string()), // Cursor on
                                    '6' => Some("\x1b7".to_string()),     // Remember current cursor position
                                     _  => Some("".to_string())           // Unknown, ignore
                                }
                            }
                        },
                        'C' => {
                            if self.buffer_len < 3 {
                                None // We need more buffer
                            } else {
                                match self.buffer[2] as char {
                                    '0' => Some("\x1b[27m".to_string()),  // Reverse video stop
                                    '1' => Some("\x1b[22m".to_string()),  // Half intensity stop
                                    '2' => Some("\x1b[25m".to_string()),  // Blinking stop
                                    '3' => Some("\x1b[24m".to_string()),  // Underline stop
                                    '4' => Some("\x1b[?25l".to_string()), // Cursor off
                                    '6' => Some("\x1b7".to_string()),     // Return to last remembered cursor position
                                     _  => Some("".to_string())           // Unknown, ignore
                                }
                            }
                        },
                        'G' => {
                            if self.buffer_len < 3 {
                                None // We need more buffer
                            } else {
                                match self.buffer[2] as char {
                                    '0' => Some("\x1b[27m".to_string()),  // Reset to standard video: doing reverse video stop
                                    '4' => Some("\x1b[7m".to_string()),   // Reversing of disignated area: doing reverse video start
                                     _  => Some("".to_string())           // Unknown, ignore
                                }
                            }
                        },
                        'T' => Some("\x1b[K".to_string()),    // Erase to end of line
                        '(' => Some("\x1b[2m".to_string()),   // Write protect start (reduced intensity): doing half intensity start
                        ')' => Some("\x1b[22m".to_string()),  // Write protect stop (reduced intensity): doing half intensity stop
                        '=' => {
                            if self.buffer_len < 4 {
                                None // We need more buffer
                            } else {
                                let r32 = self.buffer[2];
                                let c32 = self.buffer[3];
                                if r32 < 32 || c32 < 32 {
                                    Some("".to_string()) // Invalid numbers, ignore
                                } else {
                                    Some(format!("\x1b[{};{}H", r32-31, c32-31))
                                }
                            }
                        },
                        _  => Some("".to_string()) // Unknown, ignore
                        //_  => Some(format!("[Unknown escape code -{}-]", self.buffer[1] as char)) // Unknown
                    }
                }
            },    
            //_ => Some(format!("0x{:02x} ", self.buffer[0] /*as char*/)) // Write the char
            _ => Some(format!("{}", self.buffer[0] as char)) // Write the char
        }
    }

    pub fn translate(&mut self, ch: u8) -> Option<String> {
        self.buffer[self.buffer_len] = ch & 0x7f; // Only 7 bits ASCII
        self.buffer_len += 1;

        let conversion = self.conversion();
        if conversion.is_some() {
            self.buffer_len = 0;
        }
        conversion
    }     
}

