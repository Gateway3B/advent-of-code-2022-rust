use std::{fs::read_to_string, ops::ControlFlow, path::Path};

#[derive(Debug)]
struct ElfSectionRange {
    range_start: u32,
    range_end: u32,
}

impl ElfSectionRange {
    fn contains(&self, other: &Self) -> bool {
        let contains_start = self.range_start <= other.range_start;
        let contains_end = self.range_end >= other.range_end;
        contains_start && contains_end
    }
}

#[derive(Debug)]
struct ElfGroup {
    elf_section_ranges: Vec<ElfSectionRange>,
}

impl ElfGroup {
    fn has_one_range_fully_contain_another(&self) -> bool {
        let mut contains = false;
        self.elf_section_ranges
            .iter()
            .enumerate()
            .try_for_each(|(index, section_range_1)| {
                self.elf_section_ranges
                    .iter()
                    .skip(index + 1)
                    .try_for_each(|section_range_2| {
                        if section_range_1.contains(section_range_2)
                            || section_range_2.contains(section_range_1)
                        {
                            contains = true;
                            return ControlFlow::Break(());
                        }
                        ControlFlow::Continue(())
                    });

                if contains {
                    return ControlFlow::Break(());
                } else {
                    return ControlFlow::Continue(());
                }
            });

        contains
    }
}

fn read_elf_groups(path: &Path) -> Result<Vec<ElfGroup>, &str> {
    let lines = if let Ok(lines) = read_to_string(path) {
        lines
    } else {
        return Err("Error reading file");
    };
    let mut elf_groups: Vec<ElfGroup> = Vec::new();
    for line in lines.lines() {
        let mut elf_group = ElfGroup {
            elf_section_ranges: Vec::new(),
        };
        let mut reading_range_start = true;
        let mut range_start: u32 = 0;
        let mut range_end: u32 = 0;

        line.chars()
            .chain([','].into_iter())
            .for_each(|char| match char {
                '0'..='9' => {
                    if let Some(digit) = char.to_digit(10) {
                        if reading_range_start {
                            range_start *= 10;
                            range_start += digit;
                        } else {
                            range_end *= 10;
                            range_end += digit;
                        }
                    }
                }
                '-' | ',' => {
                    reading_range_start = !reading_range_start;
                    if char == ',' {
                        elf_group.elf_section_ranges.push(ElfSectionRange {
                            range_start,
                            range_end,
                        });
                        range_start = 0;
                        range_end = 0;
                    }
                }
                _ => (),
            });

        elf_groups.push(elf_group);
    }

    Ok(elf_groups)
}

pub fn day4() {
    let elf_groups = read_elf_groups(Path::new("elf-groups.txt")).unwrap();
    let contains_count = elf_groups.iter().fold(0, |mut sum, elf_group| {
        if elf_group.has_one_range_fully_contain_another() {
            sum += 1;
        }

        sum
    });

    println!("{contains_count}");
}
