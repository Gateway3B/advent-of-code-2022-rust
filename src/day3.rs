use std::{collections::HashSet, fs::read_to_string, ops::ControlFlow, path::Path};

trait Priority {
    fn to_priority(&self) -> Option<u32>;
}

impl Priority for char {
    fn to_priority(&self) -> Option<u32> {
        match self {
            'A'..='Z' => {
                let digit = self.clone() as u32;
                Some(digit - 65 + 27)
            }

            'a'..='z' => {
                let digit = self.clone() as u32;
                Some(digit - 97 + 1)
            }
            _ => None,
        }
    }
}

struct Compartment {
    contents: Vec<char>,
}

struct Rucksack {
    compartments: Vec<Compartment>,
}

impl Rucksack {
    fn get_packing_errors_priority(&self) -> u32 {
        let mut errors: HashSet<char> = HashSet::new();
        self.compartments
            .get(0)
            .unwrap()
            .contents
            .iter()
            .for_each(|char| {
                if self.compartments.get(1).unwrap().contents.contains(char) {
                    errors.insert(*char);
                }
            });

        errors.iter().fold(0, |priorities, char| {
            priorities + char.to_priority().unwrap_or_default()
        })
    }

    fn get_packing_errors_priority_anysize(&self) -> u32 {
        let mut errors: HashSet<char> = HashSet::new();
        let mut compartment_sets: Vec<HashSet<char>> = Vec::new();

        self.compartments.iter().skip(1).for_each(|compartment| {
            let compartment_set = HashSet::from_iter(compartment.contents.iter().cloned());
            compartment_sets.push(compartment_set);
        });

        self.compartments
            .iter()
            .enumerate()
            .for_each(|(index, compartment)| {
                compartment.contents.iter().for_each(|char| {
                    compartment_sets
                        .iter()
                        .skip(index)
                        .try_for_each(|compartment_set| {
                            if compartment_set.contains(char) {
                                errors.insert(*char);
                                return ControlFlow::Break(());
                            }
                            ControlFlow::Continue(())
                        });
                });
            });

        errors.iter().fold(0, |priorities, char| {
            priorities + char.to_priority().unwrap_or_default()
        })
    }
}

fn read_rucksacks_contents(
    path: &Path,
    rucksack_container_count: usize,
) -> Result<Vec<Rucksack>, String> {
    let mut rucksacks: Vec<Rucksack> = Vec::new();

    for line in read_to_string(path).unwrap().lines() {
        if line.len() % rucksack_container_count != 0 {
            return Err(format!(
                "A rucksack is not divisible into {rucksack_container_count} equal containers"
            ));
        }
        let compartment_size = line.len() / rucksack_container_count;
        let chars: Vec<char> = line.chars().collect();

        let mut rucksack = Rucksack {
            compartments: Vec::new(),
        };

        for i in 0..2 {
            rucksack.compartments.push(Compartment {
                contents: (&chars[(i * compartment_size)..((i + 1) * compartment_size)]).to_vec(),
            });
        }

        rucksacks.push(rucksack);
    }

    Ok(rucksacks)
}

pub fn day3() {
    let rucksacks = read_rucksacks_contents(Path::new("rucksacks-contents.txt"), 2).unwrap();
    let priorities: Vec<u32> = rucksacks
        .iter()
        .map(|rucksack| rucksack.get_packing_errors_priority_anysize())
        .collect();

    let priorities_sum: u32 = priorities.iter().sum();

    print!("{:?}", priorities_sum);
}
