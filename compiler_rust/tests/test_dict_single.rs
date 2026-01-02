// Test single-line dict vs multi-line
let single_line_dict = dict!{"fn" => "keyword1", "let" => "keyword2"};

println!("{}", "Single-line dict:");
println!("{}", single_line_dict);
println!("{}", "fn in single_line_dict: " + format!("{}", "fn" in single_line_dict));

fn main() {
    println!("{}", "Done!");
}