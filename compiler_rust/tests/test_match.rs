// Test match expression with enum
enum Color {
    Red,
    Blue,
    Green,

}
fn get_name(c: Color)  ->  String {
    match c {
        Color.Red => "red",
        Color.Blue => "blue",
        _ => "other",

    }
}
let r = Color.Red;
let result = get_name(r);
println!("{}", "Result: " + result);

fn main() {
    println!("{}", "Done");
}