pub trait Stage {
    type Input;
    type Output;
    fn process(&self, input: Self::Input) -> Self::Output;
}

pub struct Pipeline<T> {
    data: T,
}

impl<T> Pipeline<T> {
    pub fn new(data: T) -> Self {
        Self { data }
    }
}
