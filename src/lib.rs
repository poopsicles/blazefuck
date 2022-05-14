mod tokens;
pub mod interpreter;
pub mod interact;

use crate::tokens::*;
use anyhow::{Context, Result};

fn lex(string: &str) -> Vec<Instruction> {
    let mut tokens = string
        .bytes()
        .filter_map(|x| match x {
            b'>' => Some(Instruction::POINTER_RIGHT),
            b'<' => Some(Instruction::POINTER_LEFT),
            b'+' => Some(Instruction::DATA_INCREMENT),
            b'-' => Some(Instruction::DATA_DECREMENT),
            b'.' => Some(Instruction::DATA_OUTPUT),
            b',' => Some(Instruction::DATA_INPUT),
            b'[' => Some(Instruction::CONDITIONAL_BEGIN),
            b']' => Some(Instruction::CONDITIONAL_END),
            _ => None,
        })
        .collect::<Vec<Instruction>>();

    tokens.shrink_to_fit();
    tokens
}

fn parse(instructions: &Vec<Instruction>, wait: bool) -> Result<Vec<(usize, usize)>> {
    let mut stack: Vec<(usize, usize)> = Vec::with_capacity(instructions.len() / 2);
    let mut bracket_pairs = Vec::with_capacity(instructions.len() / 2);

    for (position, command) in instructions.iter().enumerate().filter(|&(_, &command)| {
        command == Instruction::CONDITIONAL_BEGIN || command == Instruction::CONDITIONAL_END
    }) {
        match command {
            Instruction::CONDITIONAL_BEGIN => stack.push((position, 0)),
            Instruction::CONDITIONAL_END => {
                match stack.pop() {
                    Some((begin, _)) => bracket_pairs.push((begin, position)),
                    None => return Err(Error::UNMATCHED_CONDITIONAL(position)).context("Syntax error encountered")
                };
            }
            _ => (),
        }
    }

    if !stack.is_empty() {
        if !wait {
            return Err(Error::UNMATCHED_CONDITIONAL(stack[0].1 + 1)).context("Syntax error encountered");
        }

        bracket_pairs.push((stack[0].1, 0));
    }

    bracket_pairs.sort();
    bracket_pairs.shrink_to_fit();
    Ok(bracket_pairs)
}
