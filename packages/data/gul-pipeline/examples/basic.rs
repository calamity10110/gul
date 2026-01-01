use gul_pipeline::{Pipeline, Stage};

struct AddOne;
impl Stage<i32> for AddOne {
    fn process(&self, input: i32) -> i32 {
        input + 1
    }
}

struct Double;
impl Stage<i32> for Double {
    fn process(&self, input: i32) -> i32 {
        input * 2
    }
}

fn main() {
    let pipeline = Pipeline::new().add_stage(AddOne).add_stage(Double);

    let result = pipeline.execute(10); // (10 + 1) * 2 = 22
    println!("Result: {}", result);
}
