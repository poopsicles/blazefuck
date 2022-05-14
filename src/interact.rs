use anyhow::{Context, Result};
use crate::{lex, parse, tokens::*};
use std::io::Write;

pub struct Program {
    strict: bool,

    cells: Vec<u8>,
    instructions: Vec<Instruction>,

    left_bracket_map: Vec<(usize, usize)>,
    right_bracket_map: Vec<(usize, usize)>,

    cell_pointer: usize,
    instruction_pointer: usize,
}

impl Program {
    pub fn new(strict: bool) -> Program {
        Program {
            strict,
            cells: vec![0],
            instructions: vec![],
            left_bracket_map: vec![],
            right_bracket_map: vec![],
            cell_pointer: 0,
            instruction_pointer: 0,
        }
    }

    pub fn run(&mut self, debug: bool) -> Result<()> {
        'begin: loop {
            print!(">>> ");
            std::io::stdout().flush().unwrap();

            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();

            if input.trim().to_ascii_lowercase() == String::from("exit") {
                break 'begin;
            }
            
            if input.trim().to_ascii_lowercase() == String::from("cells") {
                println!("{:?}\n", self.cells);
                continue;
            }

            self.instructions.append(&mut lex(&input));
            self.left_bracket_map = parse(&self.instructions, true)?;
            self.right_bracket_map = self.left_bracket_map.clone();
            self.right_bracket_map.sort_by_key(|&(_, b)| b);

            loop {
                match self.instructions.get(self.instruction_pointer) {
                    None => break,
                    Some(&t) => match t {
                        Instruction::POINTER_RIGHT => {
                            if self.cell_pointer == 29999 && self.strict {
                                println!();
                                return Err(Error::CELL_BOUNDS_VIOLATED_RIGHT).with_context(|| {
                                    format!("Error at command #{}", self.instruction_pointer + 1)
                                });
                            }

                            if self.cell_pointer >= self.cells.len() - 1 {
                                self.cells.push(0);
                            }

                            self.cell_pointer += 1;
                        }

                        Instruction::POINTER_LEFT => {
                            if self.cell_pointer == 0 {
                                if self.strict {
                                    println!();
                                    return Err(Error::CELL_BOUNDS_VIOLATED_LEFT).with_context(
                                        || {
                                            format!(
                                                "Error at command #{}",
                                                self.instruction_pointer + 1
                                            )
                                        },
                                    );
                                } else {
                                    self.cell_pointer = self.cells.len()
                                }
                            }

                            self.cell_pointer -= 1
                        }

                        Instruction::DATA_INCREMENT => match (self.cells[self.cell_pointer], self.strict)
                        {
                            (255, false) => self.cells[self.cell_pointer] = 0,
                            (255, true) => {
                                println!();
                                return Err(Error::CELL_DATA_OVERFLOW).with_context(|| {
                                    format!("Error at command #{}", self.instruction_pointer + 1)
                                })
                            }
                            _ => self.cells[self.cell_pointer] += 1,
                        },

                        Instruction::DATA_DECREMENT => match (self.cells[self.cell_pointer], self.strict)
                        {
                            (0, false) => self.cells[self.cell_pointer] = 255,
                            (0, true) => {
                                println!();
                                return Err(Error::CELL_DATA_UNDERFLOW).with_context(|| {
                                    format!("Error at command #{}", self.instruction_pointer + 1)
                                })
                            }
                            _ => self.cells[self.cell_pointer] -= 1,
                        },

                        Instruction::DATA_OUTPUT => {
                            print!("{}", self.cells[self.cell_pointer] as char);
                            std::io::stdout().flush().unwrap();
                        }

                        Instruction::DATA_INPUT => {
                            print!("\n? ");
                            std::io::stdout().flush().unwrap();

                            let mut input = String::new();
                            std::io::stdin().read_line(&mut input).unwrap_or_default();

                            input = input.replace("\r", "");
                            let input = input.as_bytes();

                            let byte = input[0];
                            let remaining = &input[1..];

                            match remaining.get(1) {
                                Some(10) | None => self.cells[self.cell_pointer] = byte,
                                _ => {
                                    return Err(Error::INPUT_OVERFLOW).with_context(|| {
                                        format!(
                                            "Error at command #{}",
                                            self.instruction_pointer + 1
                                        )
                                    })
                                }
                            }
                        }

                        Instruction::CONDITIONAL_BEGIN => {
                            if self.cells[self.cell_pointer] == 0 {
                                let next = self.left_bracket_map[self
                                    .left_bracket_map
                                    .binary_search_by_key(&self.instruction_pointer, |&(a, _)| a)
                                    .unwrap()]
                                .1;

                                if next == 0 {
                                    continue 'begin;
                                } else {
                                    self.instruction_pointer = next;
                                }
                            }
                        }

                        Instruction::CONDITIONAL_END => {
                            if self.cells[self.cell_pointer] != 0 {
                                self.instruction_pointer = self.right_bracket_map[self
                                    .right_bracket_map
                                    .binary_search_by_key(&self.instruction_pointer, |&(_, b)| b)
                                    .unwrap()]
                                .0;
                            }
                        }
                    },
                }
                self.instruction_pointer += 1
            }
            if debug {println!("{:?}\n", self.cells)};
        }
        Ok(())
    }
}
