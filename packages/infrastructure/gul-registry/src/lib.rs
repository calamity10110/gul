use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug)]
pub struct ServiceInstance {
    pub id: String,
    pub name: String,
    pub url: String,
    pub health_status: String,
}

#[derive(Clone)]
pub struct Registry {
    services: Arc<Mutex<HashMap<String, Vec<ServiceInstance>>>>,
}

impl Registry {
    pub fn new() -> Self {
        Self {
            services: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn register(&self, name: &str, url: &str) -> String {
        let mut map = self.services.lock().unwrap();
        let instances = map.entry(name.to_string()).or_insert(Vec::new());
        
        let id = uuid::Uuid::new_v4().to_string();
        instances.push(ServiceInstance {
            id: id.clone(),
            name: name.to_string(),
            url: url.to_string(),
            health_status: "UP".to_string(),
        });
        id
    }

    pub fn discover(&self, name: &str) -> Option<ServiceInstance> {
        let map = self.services.lock().unwrap();
        // Simple Round-Robin or random could go here. Returning first for now.
        map.get(name).and_then(|v| v.first().cloned())
    }
}
