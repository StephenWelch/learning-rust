mod entity;
mod world;
mod coord;


use std::io;
use std::io::Write;
use std::collections::HashMap;
use crate::coord::Coord;
use crate::entity::Entity;
use crate::world::{Cell, World};

macro_rules! print_flush {
    ( $($t:tt)* ) => {
        {
            let mut h = io::stdout();
            write!(h, $($t)* ).unwrap();
            h.flush().unwrap();
        }
    }
}


fn main() {
    const EMPTY_CELL: Cell = Cell::new("An empty field.",
                                         "Before you lies a barren grassland, empty as far as the eye can see",
    );

    const WORLD_SZ: usize = 128;

    let mut defs: HashMap<&str, fn()> = HashMap::new();
    let cells: [[Cell; WORLD_SZ]; WORLD_SZ] = [[EMPTY_CELL; WORLD_SZ]; WORLD_SZ];
    let mut player = Entity::new(Coord::new(0, 0), 100);

    let mut world = World::new(cells, vec![player]);

    defs.insert("test", || {print_flush!("Hello!\n")});

    let mut input = String::new();
    loop {
        print_flush!("> ");
        io::stdin().read_line(&mut input).expect("Unable to read input.");

        parse_input(&input, &defs);
        input.clear();
    }
}


fn parse_input(input: &String, defs: &HashMap<&str, fn()>) {
    let mut formatted_input = input.to_lowercase();
    formatted_input = formatted_input.strip_suffix("\n").unwrap().to_string();

    match defs.get(&*formatted_input) {
        Some(func) => {
            func()
        }
        _ => {}
    }
    print_flush!("{}", input);
}