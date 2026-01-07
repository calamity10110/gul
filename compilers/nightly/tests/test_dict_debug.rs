// Test dict storage and keys
let test_dict = dict!{
    "fn": "keyword1",
    "let": "keyword2";
};

println!("{}", "Test dict contents:");
println!("{}", test_dict);
println!("{}", "Keys:");
// Can we iterate?
// Try direct key check
let has_fn = "fn" in test_dict;
println!("{}", "fn check result: " + format!("{}", has_fn));

fn main() {
    println!("{}", "Done!");
}