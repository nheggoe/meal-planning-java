use super::{Error, Recipe};

#[allow(async_fn_in_trait)]
pub trait Repo {
    async fn create(&self, recipe: Recipe) -> Result<(), Error>;
    async fn delete(&self, name: &str) -> Result<(), Error>;
    async fn read(&self, name: &str) -> Result<Recipe, Error>;
}
