use super::Allocator;

pub struct PoolAllocator<T> {
    pool: Vec<T>,
}

impl<T> PoolAllocator<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            pool: Vec::with_capacity(capacity),
        }
    }
}

impl<T> Allocator<T> for PoolAllocator<T> {
    fn allocate(&mut self, value: T) -> &mut T {
        self.pool.push(value);
        self.pool.last_mut().unwrap()
    }

    fn deallocate(&mut self) {
        self.pool.clear();
    }
}
