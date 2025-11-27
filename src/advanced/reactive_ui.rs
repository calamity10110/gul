// Reactive UI system for Universal Language
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct ReactiveState<T: Clone> {
    value: Arc<Mutex<T>>,
    subscribers: Arc<Mutex<Vec<Box<dyn Fn(&T) + Send + Sync>>>>,
}

impl<T: Clone> ReactiveState<T> {
    pub fn new(initial: T) -> Self {
        ReactiveState {
            value: Arc::new(Mutex::new(initial)),
            subscribers: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn get(&self) -> T {
        self.value.lock().unwrap().clone()
    }

    pub fn set(&self, new_value: T) {
        {
            let mut value = self.value.lock().unwrap();
            *value = new_value.clone();
        }

        // Notify all subscribers
        let subscribers = self.subscribers.lock().unwrap();
        for callback in subscribers.iter() {
            callback(&new_value);
        }
    }

    pub fn subscribe<F>(&self, callback: F)
    where
        F: Fn(&T) + Send + Sync + 'static,
    {
        let mut subscribers = self.subscribers.lock().unwrap();
        subscribers.push(Box::new(callback));
    }
}

#[derive(Debug, Clone)]
pub struct Component {
    pub name: String,
    pub props: HashMap<String, String>,
    pub state: HashMap<String, String>,
    pub children: Vec<Component>,
}

impl Component {
    pub fn new(name: String) -> Self {
        Component {
            name,
            props: HashMap::new(),
            state: HashMap::new(),
            children: Vec::new(),
        }
    }

    pub fn with_prop(mut self, key: String, value: String) -> Self {
        self.props.insert(key, value);
        self
    }

    pub fn with_state(mut self, key: String, value: String) -> Self {
        self.state.insert(key, value);
        self
    }

    pub fn add_child(&mut self, child: Component) {
        self.children.push(child);
    }

    pub fn render(&self) -> String {
        let mut output = format!("<{}", self.name);

        for (key, value) in &self.props {
            output.push_str(&format!(" {}=\"{}\"", key, value));
        }

        if self.children.is_empty() {
            output.push_str(" />");
        } else {
            output.push('>');
            for child in &self.children {
                output.push_str(&child.render());
            }
            output.push_str(&format!("</{}>", self.name));
        }

        output
    }
}

pub struct ReactiveUI {
    components: HashMap<String, Component>,
    event_handlers: HashMap<String, Box<dyn Fn() + Send + Sync>>,
}

impl Default for ReactiveUI {
    fn default() -> Self {
        Self::new()
    }
}

impl ReactiveUI {
    pub fn new() -> Self {
        ReactiveUI {
            components: HashMap::new(),
            event_handlers: HashMap::new(),
        }
    }

    pub fn register_component(&mut self, id: String, component: Component) {
        self.components.insert(id, component);
    }

    pub fn get_component(&self, id: &str) -> Option<&Component> {
        self.components.get(id)
    }

    pub fn on_event<F>(&mut self, event_name: String, handler: F)
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.event_handlers.insert(event_name, Box::new(handler));
    }

    pub fn trigger_event(&self, event_name: &str) {
        if let Some(handler) = self.event_handlers.get(event_name) {
            handler();
        }
    }

    pub fn render_all(&self) -> String {
        let mut output = String::new();
        for (id, component) in &self.components {
            output.push_str(&format!("<!-- {} -->\n", id));
            output.push_str(&component.render());
            output.push('\n');
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reactive_state() {
        let state = ReactiveState::new(42);
        assert_eq!(state.get(), 42);

        state.set(100);
        assert_eq!(state.get(), 100);
    }

    #[test]
    fn test_reactive_state_subscription() {
        let state = ReactiveState::new(0);
        let counter = Arc::new(Mutex::new(0));
        let counter_clone = counter.clone();

        state.subscribe(move |_value| {
            let mut c = counter_clone.lock().unwrap();
            *c += 1;
        });

        state.set(1);
        state.set(2);

        assert_eq!(*counter.lock().unwrap(), 2);
    }

    #[test]
    fn test_component_creation() {
        let component = Component::new("Button".to_string())
            .with_prop("text".to_string(), "Click me".to_string());

        assert_eq!(component.name, "Button");
        assert_eq!(component.props.get("text").unwrap(), "Click me");
    }

    #[test]
    fn test_component_render() {
        let component = Component::new("div".to_string())
            .with_prop("class".to_string(), "container".to_string());

        let rendered = component.render();
        assert!(rendered.contains("div"));
        assert!(rendered.contains("class=\"container\""));
    }

    #[test]
    fn test_component_with_children() {
        let mut parent = Component::new("div".to_string());
        let child =
            Component::new("span".to_string()).with_prop("text".to_string(), "Hello".to_string());

        parent.add_child(child);

        let rendered = parent.render();
        assert!(rendered.contains("<div"));
        assert!(rendered.contains("<span"));
        assert!(rendered.contains("</div>"));
    }

    #[test]
    fn test_reactive_ui() {
        let mut ui = ReactiveUI::new();
        let component = Component::new("Button".to_string());

        ui.register_component("btn1".to_string(), component);

        assert!(ui.get_component("btn1").is_some());
    }

    #[test]
    fn test_event_handling() {
        let mut ui = ReactiveUI::new();
        let clicked = Arc::new(Mutex::new(false));
        let clicked_clone = clicked.clone();

        ui.on_event("click".to_string(), move || {
            let mut c = clicked_clone.lock().unwrap();
            *c = true;
        });

        ui.trigger_event("click");

        assert!(*clicked.lock().unwrap());
    }

    #[test]
    fn test_render_all() {
        let mut ui = ReactiveUI::new();
        let component = Component::new("div".to_string());

        ui.register_component("root".to_string(), component);

        let rendered = ui.render_all();
        assert!(rendered.contains("<!-- root -->"));
        assert!(rendered.contains("<div"));
    }
}
