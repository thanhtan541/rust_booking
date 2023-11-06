use core::fmt::Debug;
use std::error::Error;

pub trait Reposity<T>
where
    T: Debug,
{
    fn find_by_id(&self, id: i32) -> Result<Option<T>, Box<dyn Error>>;

    fn find_all(&self) -> Result<Vec<T>, Box<dyn Error>>;

    fn create(&self, entity: T) -> Result<(), Box<dyn Error>>;

    fn update(&self, entity: T) -> Result<(), Box<dyn Error>>;

    fn delete(&self, id: i32) -> Result<(), Box<dyn Error>>;
}
