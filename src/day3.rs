use std::{collections::HashSet, fs::read_to_string, path::Path};

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
}

fn read_rucksacks_contents(path: &Path) -> Vec<Rucksack> {
    let mut rucksacks: Vec<Rucksack> = Vec::new();

    for line in read_to_string(path).unwrap().lines() {
        let compartment_size = line.len() / 2;
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

    rucksacks
}

pub fn day3() {
    let rucksacks = read_rucksacks_contents(Path::new("rucksacks-contents.txt"));
    let priorities: Vec<u32> = rucksacks
        .iter()
        .map(|rucksack| rucksack.get_packing_errors_priority())
        .collect();

    let priorities_sum: u32 = priorities.iter().sum();

    print!("{:?}", priorities_sum);
}
