pub use jrny::*;

mod jrny {
    use crate::Coord;
    use crate::entity::Entity;

    const WORLD_SIZE: usize = 32;

    #[derive(Debug)]
    pub struct Cell<'a> {
        short_desc: &'a str,
        long_desc: &'a str,
    }

    impl Cell<'_> {
        pub const fn new<'a>(short_desc: &'a str, long_desc: &'a str) -> Cell<'a> {
            return Cell {
                short_desc,
                long_desc
            }
        }
    }

    #[derive(Debug)]
    pub struct World<'a, const WORLD_SZ: usize> {
        cells: [[&'a mut Cell<'a>; WORLD_SZ]; WORLD_SZ],
        entities: Vec<Entity>,
    }

    impl World<'_, WORLD_SIZE> {
        pub fn new<'a>(cells: [[&'a mut Cell<'a>; WORLD_SIZE]; WORLD_SIZE], entities: Vec<Entity>) -> World<'a, WORLD_SIZE> {
            World{cells, entities}
        }

        pub fn spawn_entity(&mut self, entity: Entity) {
            self.entities.push(entity);
        }

        pub fn get_cell(&mut self, pos: Coord) -> &'_ mut Cell {
            self.cells[pos.x() as usize][pos.y() as usize]
        }

        pub fn get_entities(&mut self, pos: Coord) -> Vec<&mut Entity> {
            self.entities.into_iter().filter(|e| e.get_pos() == pos).collect()
        }
    }
}