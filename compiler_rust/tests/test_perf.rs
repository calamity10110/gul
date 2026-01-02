use crate::std::io;

fn tokenize_min(source: String)  ->  i64 {
    let mut l_pos = i64(0);
    let mut l_len = (source).len();
    let mut count = i64(0);

    while l_pos < l_len {
        let ch = source[l_pos];
        if ch == " " || ch == "\t" || ch == "\n" || ch == "\r" {
            l_pos = l_pos + 1;
            continue;
        }
        if ch == "
            while l_pos < l_len && source[l_pos] != "\n" {
                l_pos = l_pos + 1;
        }
            continue;
        l_pos = l_pos + 1;
        count = count + 1;
    }
    return count;

}
fn main() {
    let source = "; // test\nlet x = 1\n"
    println!("{}", "START");
    let count = tokenize_min(source);
    println!("{}", "DONE: " + format!("{}", count));

}
main();