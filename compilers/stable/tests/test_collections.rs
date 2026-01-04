// Test GUL Collection Features
// Testing all collection syntaxes as specified

// Basic collections with let/var
let numbers: list = [1, 2, 3, 4, 5]; // Immutable
let mut items: list = [1, 2, 3]; // Mutable

let labels: set = {"a", "b"}; // Immutable
let mut tags: set = {"rust", "python"}; // Mutable

let user: dict = {name: "Alice", age: 25}; // Immutable
let mut cfg: dict = {host: "localhost", port: 8080}; // Mutable

// Type constructors
let name = String("Alice");
let age = i64(30);
let score = f64(95.5);
let active = bool(true);

// Collection type constructors
let nums = Vec(1, 2, 3);
let mut mut_items = Vec(1, 2, 3, "four");

let point = @tuple(10, 20);
let tag_set = HashSet{"a", "b", "c"};
let config = dict!{
    host: "localhost",
    port: 8080,
    debug: true,
};

// Collection methods (syntax test)
// mut_items.insertbefore(0)
// mut_items.insertafter("Five")
// mut_items.add(6)
// mut_items.remove(1)

// Access
let first = numbers[0];
let last = numbers[-1];

// Mutable dict operations
let mut my_cfg = dict!{
    host: "localhost",
    port: 8080,
};

// my_cfg.add(ssl: true)
// my_cfg.remove(port)

println!("{}", "All syntax tests passed!");