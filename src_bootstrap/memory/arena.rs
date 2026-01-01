use super::Allocator;

pub struct Arena<T> {
    chunks: Vec<Vec<T>>,
    current_chunk: usize,
    chunk_size: usize,
}

impl<T> Arena<T> {
    pub fn new(chunk_size: usize) -> Self {
        Self {
            chunks: vec![Vec::with_capacity(chunk_size)],
            current_chunk: 0,
            chunk_size,
        }
    }
}

impl<T> Allocator<T> for Arena<T> {
    fn allocate(&mut self, value: T) -> &mut T {
        if self.chunks[self.current_chunk].len() >= self.chunk_size {
            self.chunks.push(Vec::with_capacity(self.chunk_size));
            self.current_chunk += 1;
        }

        let chunk = &mut self.chunks[self.current_chunk];
        chunk.push(value);

        // This is a bit tricky in Rust due to lifetimes.
        // In a real implementation, we'd likely use UnsafeCell or raw pointers
        // to return a reference that lives as long as the Arena.
        // For this safe implementation, we'll just return the last element.
        chunk.last_mut().unwrap()
    }

    fn deallocate(&mut self) {
        self.chunks.clear();
        self.chunks.push(Vec::with_capacity(self.chunk_size));
        self.current_chunk = 0;
    }
}
