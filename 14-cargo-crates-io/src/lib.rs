//! # Art
//!
//! a library for modeling artistic concepts


pub mod colors {

    pub enum Primary {
        Red,
        Yellow,
        Blue,
    }

    pub enum Secondary {
        Orange,
        Green,
        Purple,
    }

    pub enum RGB {
        Red,
        Green,
        Blue,
    }
}

pub mod utils {
    use crate::colors::*;

    /// combines two Primary colors into a Secondary color
    pub fn mix(c1: Primary, c2: Primary,) -> Secondary {
        Secondary::Orange
    }
}

