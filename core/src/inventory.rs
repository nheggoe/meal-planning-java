mod repo;

pub use repo::Repo;

/// A named place that holds ingredients, e.g. "fridge" or "freezer".
pub struct Storage {
    pub name: String,
}

pub struct Ingredient {
    pub name: String,
    pub amount: f64,
    pub unit: Measurement,
}

pub enum Measurement {
    Gram,
    Liter,
    Piece,
}

#[derive(Debug)]
pub enum Error {
    NotFound,
}
