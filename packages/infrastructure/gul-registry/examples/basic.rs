use gul_registry::Registry;

fn main() {
    let registry = Registry::new();
    let id = registry.register("auth-service", "http://localhost:8081");
    println!("Registered service {}", id);
    
    let instance = registry.discover("auth-service").unwrap();
    println!("Discovered: {} at {}", instance.name, instance.url);
}
