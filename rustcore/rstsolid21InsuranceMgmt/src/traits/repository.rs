/// Generic repository trait following the Repository pattern
pub trait Repository<T, ID> {
    fn get_all(&self) -> Vec<T>;
    fn get_by_id(&self, id: ID) -> Option<T>;
    fn add(&mut self, item: T) -> ID;
    fn update(&mut self, id: ID, item: T) -> bool;
    fn delete(&mut self, id: ID) -> bool;
}
