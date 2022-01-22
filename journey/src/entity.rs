pub use jrny::*;

mod jrny {
    use crate::coord::Coord;

    #[derive(Debug, Clone, Copy)]
    pub struct Entity {
        pos: Coord,
        health: i32,
    }

    impl Entity {
        pub fn new(pos: Coord, health: i32) -> Entity {
            Entity { pos, health }
        }

        pub fn set_pos(&mut self, pos: Coord) {
            self.pos = pos
        }

        pub fn get_pos(&self) -> Coord {
            self.pos
        }

        pub fn north(&mut self) -> Coord {
            self.pos += Coord::new(0, 1);
            self.pos
        }

        pub fn east(&mut self) -> Coord {
            self.pos += Coord::new(1, 0);
            self.pos
        }

        pub fn west(&mut self) -> Coord {
            self.pos += Coord::new(-1, 0);
            self.pos
        }

        pub fn south(&mut self) -> Coord {
            self.pos += Coord::new(0, -1);
            self.pos
        }
    }
}