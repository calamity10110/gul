pub mod docker {
    pub struct Container {
        pub id: String,
        pub image: String,
        pub status: String,
    }

    impl Container {
        pub fn new(image: &str) -> Self {
            Self {
                id: uuid::Uuid::new_v4().to_string(), // Requires uuid crate, but I will mock for now
                image: image.to_string(),
                status: "created".to_string(),
            }
        }

        pub fn start(&mut self) {
            self.status = "running".to_string();
        }
    }
}

pub mod kubernetes {
    pub struct Pod {
        pub name: String,
        pub namespace: String,
        pub containers: Vec<String>,
    }

    impl Pod {
        pub fn new(name: &str, namespace: &str) -> Self {
            Self {
                name: name.to_string(),
                namespace: namespace.to_string(),
                containers: Vec::new(),
            }
        }
    }
}

pub mod monitoring {
    pub struct Metric {
        pub name: String,
        pub value: f64,
        pub labels: std::collections::HashMap<String, String>,
    }
}
