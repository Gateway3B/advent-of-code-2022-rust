use std::{fs::read_to_string, ops::ControlFlow, path::Path};

#[derive(Debug)]
struct CrateStack {
    crates: Vec<char>,
}

#[derive(Debug)]
struct CraneInstruction {
    origin_stack: usize,
    destination_stack: usize,
    crates_to_move: usize,
}

struct Ship {
    crate_stacks: Vec<CrateStack>,
    crane_instructions: Vec<CraneInstruction>,
}

impl Ship {
    fn execute_instructions(&mut self) {
        self.crane_instructions
            .iter()
            .for_each(|crane_instruction| {
                for _ in 0..crane_instruction.crates_to_move {
                    if let Some(origin_stack) = self
                        .crate_stacks
                        .get_mut(crane_instruction.origin_stack - 1)
                    {
                        if let Some(crate_unit) = origin_stack.crates.pop() {
                            if let Some(destination_stack) = self
                                .crate_stacks
                                .get_mut(crane_instruction.destination_stack - 1)
                            {
                                destination_stack.crates.push(crate_unit);
                            }
                        }
                    }
                }
                println!("{:?}", self.crate_stacks);
            });
    }

    fn tops_of_stacks(&self) -> Vec<&char> {
        let mut tops_of_stacks = Vec::new();
        self.crate_stacks.iter().for_each(|crate_stack| {
            if let Some(crate_unit) = crate_stack.crates.last() {
                tops_of_stacks.push(crate_unit);
            }
        });
        tops_of_stacks
    }
}

fn read_ship_state_and_instructions(path: &Path) -> Result<Ship, &str> {
    let lines = if let Ok(lines) = read_to_string(path) {
        lines
    } else {
        return Err("Error reading file");
    };

    let mut reached_instructions = false;

    let mut crate_stacks: Vec<CrateStack> = Vec::new();
    let mut crane_instructions: Vec<CraneInstruction> = Vec::new();

    lines.lines().into_iter().try_for_each(|line| {
        if line.len() == 0 {
            reached_instructions = true;
            return ControlFlow::<()>::Continue(());
        }

        if crate_stacks.len() == 0 {
            for _ in 0..=(line.len() / 4) {
                crate_stacks.push(CrateStack { crates: Vec::new() });
            }
        }

        if !reached_instructions {
            let mut morpheme_index = 0;
            let mut token_index = 0;

            line.chars().for_each(|char| {
                if morpheme_index == 1 && ('A'..='Z').contains(&char) {
                    if let Some(crate_stack) = crate_stacks.get_mut(token_index) {
                        if !(char == ' ') {
                            crate_stack.crates.insert(0, char);
                        }
                    };
                }

                morpheme_index += 1;
                if morpheme_index % 4 == 0 {
                    token_index += 1;
                    morpheme_index = 0;
                }
            });
        } else {
            let mut crates_to_move: usize = 0;
            let mut origin_stack: usize = 0;
            let mut destination_stack: usize = 0;

            let mut num_buffer: usize = 0;

            line.chars().chain([' '].into_iter()).for_each(|char| {
                if ('0'..='9').contains(&char) {
                    if let Some(digit) = char.to_digit(10) {
                        num_buffer *= 10;
                        num_buffer += digit as usize;
                    }
                } else {
                    if crates_to_move == 0 {
                        crates_to_move = num_buffer;
                    } else if origin_stack == 0 {
                        origin_stack = num_buffer;
                    } else if destination_stack == 0 {
                        destination_stack = num_buffer;
                    }

                    num_buffer = 0;
                }
            });

            crane_instructions.push(CraneInstruction {
                crates_to_move,
                origin_stack,
                destination_stack,
            });
        }

        ControlFlow::Continue(())
    });

    return Ok(Ship {
        crane_instructions,
        crate_stacks,
    });
}

pub fn day5() {
    let mut ship =
        read_ship_state_and_instructions(Path::new("ship-state-and-instructions.txt")).unwrap();

    println!("{:?}", ship.crate_stacks);
    println!("{:?}", ship.crane_instructions);

    ship.execute_instructions();

    let tops_of_stacks = ship.tops_of_stacks();

    println!("{:?}", tops_of_stacks);
}
