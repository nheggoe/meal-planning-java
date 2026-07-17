mod repo;

pub use repo::Repo;

pub struct Recipe {
    pub name: String,
    pub steps: Vec<Step>,
}

pub struct Step {
    pub description: String,
}

#[derive(Debug)]
pub enum Error {
    NotFound,
}
