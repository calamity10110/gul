pub mod arena;
pub mod pool;

pub trait Allocator<T> {
    fn allocate(&mut self, value: T) -> &mut T;
    fn deallocate(&mut self);
}
