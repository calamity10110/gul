pub enum ComponentType {
    Button,
    Card,
    Modal,
}

pub struct Component {
    pub id: String,
    pub ctype: ComponentType,
    pub props: std::collections::HashMap<String, String>,
}

impl Component {
    pub fn button(label: &str) -> Self {
        let mut props = std::collections::HashMap::new();
        props.insert("label".to_string(), label.to_string());
        Self {
            id: "btn".to_string(),
            ctype: ComponentType::Button,
            props,
        }
    }

    pub fn card(title: &str, content: &str) -> Self {
        let mut props = std::collections::HashMap::new();
        props.insert("title".to_string(), title.to_string());
        props.insert("content".to_string(), content.to_string());
        Self {
            id: "card".to_string(),
            ctype: ComponentType::Card,
            props,
        }
    }

    pub fn render_html(&self) -> String {
        match self.ctype {
            ComponentType::Button => format!(
                "<button>{}</button>",
                self.props.get("label").unwrap_or(&"".to_string())
            ),
            ComponentType::Card => format!(
                "<div class='card'><h3>{}</h3><p>{}</p></div>",
                self.props.get("title").unwrap_or(&"".to_string()),
                self.props.get("content").unwrap_or(&"".to_string())
            ),
            _ => "<div></div>".to_string(),
        }
    }
}
