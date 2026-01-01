use gul_ui::Component;

fn main() {
    let card = Component::card("Welcome", "Hello GUL Users");
    println!("{}", card.render_html());
}
