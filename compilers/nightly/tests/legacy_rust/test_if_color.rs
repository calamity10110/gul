// Test using if/elif instead of match
enum Color {
    Red,
    Blue,
    Green,

}
fn get_name(c: Color)  ->  String {
    if c == Color.Red {
        return "red";
    }
    else if c == Color.Blue {
        return "blue";
    }
    else {
        return "other";

    }
}
let r = Color.Red;
let result = get_name(r);
println!("{}", "Result: " + result);

fn main() {
    println!("{}", "Done");
}