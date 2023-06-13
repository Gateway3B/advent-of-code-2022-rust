use std::{fs::read_to_string, path::Path};

struct Elf {
    food_items: Vec<FoodItem>,
}

struct FoodItem {
    calories: i32,
}

impl Elf {
    fn total_food_calories(&self) -> i32 {
        self.food_items
            .iter()
            .fold(0, |total, food_item| total + food_item.calories)
    }
}

fn read_elf_calorie_list(path: &Path) -> Vec<Elf> {
    let mut elves: Vec<Elf> = Vec::new();
    let mut elf_index = 0;
    for line in read_to_string(path).unwrap_or_default().lines() {
        let elf = match elves.get_mut(elf_index) {
            Some(elf) => elf,
            None => {
                elves.push(Elf {
                    food_items: Vec::new(),
                });
                elves.get_mut(elf_index).unwrap()
            }
        };

        match line {
            "" => elf_index += 1,
            line => {
                if let Ok(calories) = line.parse::<i32>() {
                    elf.food_items.push(FoodItem { calories })
                }
            }
        }
    }

    elves
}

pub fn day1() {
    let elves = read_elf_calorie_list(Path::new("elf-calorie-list.txt"));
    let largest_calories_elf =
        elves
            .iter()
            .enumerate()
            .fold((0, 0), |largest_calories_elf, elf| {
                let total_elf_calories = elf.1.total_food_calories();
                if total_elf_calories > largest_calories_elf.1 {
                    return (elf.0, total_elf_calories);
                }
                largest_calories_elf
            });

    print!("{:?}", largest_calories_elf)
}
