//! # Notes
//! - https://piazza.com/class_profile/get_resource/hzkq9i9o1ec222/i08j0jat3m63mh
//! - https://pp.info.uni-karlsruhe.de/uploads/publikationen/braun13cc.pdf
use std::collections::{HashMap, HashSet};

use crate::instruction::{Instruction, Label, ThreeOp, Value};

/// Insert jump instructions to the end of each basic block.
fn insert_implicit_jumps(program: &Vec<Instruction>) -> Vec<Instruction> {
    // let mut program = program.iter().peekable();
    let mut result: Vec<Instruction> = Vec::new();
    let mut label_count = 0;

    for line in program.windows(2) {
        result.push(line[0].clone()); // we always push the current line
        match line {
            [Instruction::Jump(_), _] | [Instruction::Bnz(_, _), Instruction::Jump(_)] => {}
            [Instruction::Bnz(_, label), Instruction::Label(label2)] if label != label2 => {
                result.push(Instruction::Jump(label2.clone()));
            }
            [Instruction::Bnz(_, _), _] => {
                let label = Label {
                    name: format!("temp_{label_count}"),
                    params: Vec::new(),
                };
                label_count += 1;
                result.push(Instruction::Jump(label.clone()));
                result.push(Instruction::Label(label));
            }
            [_, Instruction::Label(label)] => {
                result.push(Instruction::Jump(label.clone()));
            }
            _ => {}
        }
    }
    if let Some(last) = program.last() {
        result.push(last.clone());
    }
    result
}

fn next_value(counter: &mut usize) -> Value {
    let result = Value(*counter);
    *counter += 1;
    result
}

fn find_basic_blocks(program: &[Instruction]) {
    let mut blocks: Vec<(usize, usize)> = Vec::new();

    let mut current = 0;
    for (i, line) in program.iter().enumerate() {
        match line {
            Instruction::Label(_) => {
                current = i;
            }
            Instruction::Jump(_) => {
                blocks.push((current, i + 1));
            }
            _ => {}
        }
    }

    for block in blocks {
        println!("{:?}", block);
    }
}

fn local_value_numbering(program: &[Instruction]) -> Vec<Instruction> {
    // maps the original value to the new value
    let mut mapping: HashMap<Value, Value> = HashMap::new();
    let mut result: Vec<Instruction> = Vec::new();
    let mut counter: usize = 1;

    for line in program {
        match line {
            Instruction::Int(dest, value) => {
                let new_value = next_value(&mut counter);
                mapping.insert(*dest, new_value);
                result.push(Instruction::Int(new_value, *value));
            }
            Instruction::Move(dest, arg) => {
                // a move is just renaming the value
                mapping.insert(
                    *dest,
                    mapping
                        .get(arg)
                        .expect(&format!("Value not defined {arg}"))
                        .clone(),
                );
            }
            Instruction::ThreeOp(_, dest, arga, argb) => {
                let new_value = next_value(&mut counter);
                mapping.insert(*dest, new_value);
                result.push(Instruction::ThreeOp(
                    ThreeOp::Add,
                    new_value,
                    mapping
                        .get(arga)
                        .expect(&format!("Value not defined {arga}"))
                        .clone(),
                    mapping
                        .get(argb)
                        .expect(&format!("Value not defined {argb}"))
                        .clone(),
                ));
            }
            _ => {}
        }
    }

    result
}

#[test]
fn test_local_numbering() {
    // variable numbering:
    // a = 0
    // b = 1
    // c = 2
    // d = 3

    let program = vec![
        // a = 42
        Instruction::Int(Value(0), 42),
        // b = a
        Instruction::Move(Value(1), Value(0)),
        // c = a + b
        Instruction::ThreeOp(ThreeOp::Add, Value(2), Value(0), Value(1)),
        // d = 23
        Instruction::Int(Value(3), 23),
        // a = c + d
        Instruction::ThreeOp(ThreeOp::Add, Value(0), Value(2), Value(3)),
    ];

    let result = local_value_numbering(&program);
    for line in result {
        println!("{}", line);
    }
}

macro_rules! label {
    ($a:expr) => {{
        Label {
            name: $a.to_owned(),
            params: Vec::new(),
        }
    }};
}

#[test]
fn test_jump_insertion() {
    let program = vec![
        Instruction::Label(label!("start")),
        Instruction::Bnz(Value(0), label!("start")),
        Instruction::Label(label!("end")),
        Instruction::Jump(label!("start")),
    ];

    let result = insert_implicit_jumps(&program);
    for line in result {
        println!("{}", line);
    }
}

#[test]
fn test_basic_blocks() {
    let program = vec![
        Instruction::Label(label!("start")),
        Instruction::Bnz(Value(0), label!("start")),
        Instruction::Label(label!("end")),
        Instruction::Jump(label!("start")),
    ];

    let result = insert_implicit_jumps(&program);
    for line in &result {
        println!("{}", line);
    }
    find_basic_blocks(&result);
}
