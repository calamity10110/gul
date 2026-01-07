// Test match expression with explicit return
enum Color {
    Red,
    Blue,
    Green,

}
fn get_name(c: Color)  ->  String {
    match c {
        Color.Red => return "red",
        Color.Blue => return "blue",
        _ => return "other",

    }
}
let r = Color.Red;
let result = get_name(r);
println!("{}", "Result: " + result);

fn main() {
    println!("{}", "Done");
}