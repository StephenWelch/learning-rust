pub use jrny::*;

mod jrny {
    use std::{cmp, ops};

    #[derive(Debug, Clone, Copy)]
    pub struct Coord(i32, i32);

    impl Coord {
        pub fn new(x: i32, y: i32) -> Coord {
            return Coord(x, y);
        }

        pub fn x(&self) -> i32 {
            self.0
        }

        pub fn y(&self) -> i32 {
            self.1
        }
    }

    impl ops::Add<Coord> for Coord {
        type Output = Self;
        fn add(self, rhs: Coord) -> Self::Output {
            Self (  self.x() + rhs.x(), self.y() + rhs.y()  )
        }
    }

    impl ops::Sub<Coord> for Coord {
        type Output = Self;
        fn sub(self, rhs: Coord) -> Self::Output {
            Self ( self.x() - rhs.x(), self.y() - rhs.y()  )
        }
    }
    
    impl ops::AddAssign for Coord {
        fn add_assign(&mut self, rhs: Self) {
            *self = *self + rhs
        }
    }
    
    impl ops::SubAssign for Coord {
        fn sub_assign(&mut self, rhs: Self) {
            *self = *self - rhs
        }
    }

    impl PartialEq<Self> for Coord {
        fn eq(&self, other: &Self) -> bool {
            self.x() == other.x() && self.y() == other.y()
        }
    }

    impl cmp::Eq for Coord {

    }
}

