pub struct AudioPlayer {
    volume: f32,
}

impl AudioPlayer {
    pub fn new() -> Self {
        Self { volume: 1.0 }
    }

    pub fn play(&self, file: &str) {
        println!("MOCK AUDIO: Playing '{}' at vol {:.1}", file, self.volume);
    }
}
