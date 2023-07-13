use std::{fs::read_to_string, path::Path};
use builder::{Built, Builder};
use builder_derive::Builder;

#[derive(Builder, Debug)]
struct Tree {
    height: u32,
    visible_left: bool,
    visible_right: bool,
    visible_top: bool,
    visible_bottom: bool
}

#[derive(Debug)]
struct Field {
    trees: Vec<u32>,
    height: u32,
    width: u32
}

#[derive(Debug)]
struct Coords {
    row: u32,
    column: u32
}

impl Field {
    fn get_column(&self, index: u32) -> u32 {
        index % self.width
    }

    fn get_row(&self, index: u32) -> u32 {
        index / self.width
    }

    fn get_coords(&self, index: u32) -> Coords {
        Coords { row: self.get_row(index), column: self.get_column(index) }
    }

    fn get_index(&self, coords: Coords) -> u32 {
        (coords.row * self.width) + coords.column
    }

    fn visible_left(&self, index: u32) -> Option<bool> {
        let coords = self.get_coords(index);
        let eval_tree_height = self.trees.get(index as usize)?;

        for column in (0..coords.column).into_iter() {
            let tree_height = self.trees.get(self.get_index(Coords { row: coords.row, column }) as usize)?;
            if tree_height >= eval_tree_height {
                return Some(false);
            }
        }

        Some(true)
    }

    fn visible_right(&self, index: u32) -> Option<bool> {
        let coords = self.get_coords(index);
        let eval_tree_height = self.trees.get(index as usize)?;
        
        for column in ((coords.column + 1)..self.width).rev() {
            let tree_height = self.trees.get(self.get_index(Coords { row: coords.row, column }) as usize)?;
            if tree_height >= eval_tree_height {
                return Some(false);
            }
        }

        Some(true)
    }

    fn visible_bottom(&self, index: u32) -> Option<bool> {
        let coords = self.get_coords(index);
        let eval_tree_height = self.trees.get(index as usize)?;
        
        for row in ((coords.row + 1)..self.height).rev() {
            let tree_height = self.trees.get(self.get_index(Coords { row, column: coords.column }) as usize)?;
            if tree_height >= eval_tree_height {
                return Some(false);
            }
        }

        Some(true)
    }

    fn visible_top(&self, index: u32) -> Option<bool> {
        let coords = self.get_coords(index);
        let eval_tree_height = self.trees.get(index as usize)?;

        for row in (0..coords.row).into_iter() {
            let tree_height = self.trees.get(self.get_index(Coords { row, column: coords.column }) as usize)?;
            if tree_height >= eval_tree_height {
                return Some(false);
            }
        }

        Some(true)
    }

    fn calc_visibilities(&self, index: u32) -> Option<Tree> {
        let tree_builder = Tree::builder();

        let height = self.trees.get(index as usize)?;
            
        tree_builder
            .height(height.to_owned())
            .visible_left(self.visible_left(index as u32)?)
            .visible_right(self.visible_right(index as u32)?)
            .visible_top(self.visible_top(index as u32)?)
            .visible_bottom(self.visible_bottom(index as u32)?)
            .build()
    }
}

fn read_tree_heights(path: &Path) -> Result<Field, String> {
    let lines = read_to_string(path).map_err(|_| "Error reading file".to_string())?;

    let mut field = Field {
        trees: Vec::new(),
        height: lines.lines().count() as u32,
        width: lines.lines().next().unwrap().chars().count() as u32
    };
    
    for line in lines.lines().into_iter() {
        for char in line.chars().into_iter() {
            let height = char.to_digit(10).ok_or("An input char is not a digit.".to_string())?;
            field.trees.push(height);
        }
    }
    
    Ok(field)
}

fn calc_tree_visibilities(field: &mut Field) -> Option<Vec<Tree>> {
    let mut trees = Vec::new();
    for (index, _) in field.trees.iter().enumerate() {
        trees.push(field.calc_visibilities(index as u32)?);
    };

    Some(trees)
}

fn visible_tree_count(trees: Vec<Tree>) -> Option<u32> {
    let mut count: u32 = 0;
    
    for tree in trees.iter() {
        if tree.visible_left ||
            tree.visible_right ||
            tree.visible_top ||
            tree.visible_bottom {
            count += 1;
        }
    }

    Some(count)
}

pub fn day8() {
    let mut field = read_tree_heights(Path::new("tree-heights.txt")).unwrap();
    let trees = calc_tree_visibilities(&mut field).unwrap();    
    let count = visible_tree_count(trees).unwrap();
    println!("{count}");
}