use super::{Error, Storage};

#[allow(async_fn_in_trait)]
pub trait Repo {
    async fn create(&self, name: &str) -> Result<(), Error>;
    async fn delete(&self, name: &str) -> Result<(), Error>;
    async fn rename(&self, name: &str, new_name: &str) -> Result<(), Error>;
    async fn read(&self, name: &str) -> Result<Storage, Error>;
}
