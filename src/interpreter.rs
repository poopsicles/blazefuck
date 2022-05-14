use crate::{lex, parse, tokens::*};
use anyhow::{Context, Result};
use std::{fs, io::Write};

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
    pub fn new(path: &str, strict: bool) -> Result<Program> {
        let instructions = lex(&fs::read_to_string(path)?);
        if instructions.is_empty() {
            return Err(Error::NO_COMMANDS).context("Unable to execute");
        }

        let left_bracket_map = parse(&instructions, false)?;
        let mut right_bracket_map = left_bracket_map.clone();
        right_bracket_map.sort_by_key(|&(_, b)| b);

        Ok(Program {
            strict,
            cells: vec![0; 30000],
            instructions,
            left_bracket_map,
            right_bracket_map,
            cell_pointer: 0,
            instruction_pointer: 0,
        })
    }

    pub fn run(&mut self, debug: bool) -> Result<()> {
        let mut max_cell_reached = 0;

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

                        if self.cell_pointer > 29998 {
                            self.cells.push(0);
                        }

                        self.cell_pointer += 1;

                        if debug {
                            if max_cell_reached < self.cell_pointer {
                                max_cell_reached = self.cell_pointer;
                            }
                        }
                    }

                    Instruction::POINTER_LEFT => {
                        if self.cell_pointer == 0 {
                            if self.strict {
                                println!();
                                return Err(Error::CELL_BOUNDS_VIOLATED_LEFT).with_context(|| {
                                    format!("Error at command #{}", self.instruction_pointer + 1)
                                });
                            } else {
                                self.cell_pointer = self.cells.len()
                            }
                        }

                        self.cell_pointer -= 1
                    }

                    Instruction::DATA_INCREMENT => {
                        match (self.cells[self.cell_pointer], self.strict) {
                            (255, false) => self.cells[self.cell_pointer] = 0,
                            (255, true) => {
                                println!();
                                return Err(Error::CELL_DATA_OVERFLOW).with_context(|| {
                                    format!("Error at command #{}", self.instruction_pointer + 1)
                                })
                            }
                            _ => self.cells[self.cell_pointer] += 1,
                        }
                    }

                    Instruction::DATA_DECREMENT => {
                        match (self.cells[self.cell_pointer], self.strict) {
                            (0, false) => self.cells[self.cell_pointer] = 255,
                            (0, true) => {
                                println!();
                                return Err(Error::CELL_DATA_UNDERFLOW).with_context(|| {
                                    format!("Error at command #{}", self.instruction_pointer + 1)
                                })
                            }
                            _ => self.cells[self.cell_pointer] -= 1,
                        }
                    }

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
                        let input = input.as_str();

                        let byte = input[0..1].as_bytes()[0];
                        let remaining = &input[1..];

                        if remaining != "\n" || remaining == "" {
                            return Err(Error::INPUT_OVERFLOW).with_context(|| {
                                format!("Error at command #{}", self.instruction_pointer + 1)
                            });
                        }

                        self.cells[self.cell_pointer] = byte;
                    }

                    Instruction::CONDITIONAL_BEGIN => {
                        match (self.instructions.get(self.instruction_pointer + 1), self.instructions.get(self.instruction_pointer + 2)) {
                            (Some(&Instruction::DATA_DECREMENT), Some(&Instruction::CONDITIONAL_END)) => {
                                self.cells[self.cell_pointer] = 0;
                                self.instruction_pointer += 2;
                            }

                            _ => {
                                if self.cells[self.cell_pointer] == 0 {
                                    self.instruction_pointer = self.left_bracket_map[self
                                        .left_bracket_map
                                        .binary_search_by_key(&self.instruction_pointer, |&(a, _)| a)
                                        .unwrap()]
                                    .1;
                                }
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

        if debug {
            println!("{:?}\n", &self.cells[0..max_cell_reached + 1]);
        };

        Ok(())
    }
}
