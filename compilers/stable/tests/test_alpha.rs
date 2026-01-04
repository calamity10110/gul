
fn bool is_alpha(ch: String)  ->  bool {
    let lower = "abcdefghijklmnopqrstuvwxyz";
    let res = ch in lower;
    if ! res {
        println!("{}", "FAIL: " + ch);
    }
    else {
        println!("{}", "PASS: " + ch);
    }
    return res;

}
fn main() {
    let r1 = is_alpha("f");
    println!("{}", "Result f: " + r1);

    let r2 = is_alpha("1");
    println!("{}", "Result 1: " + r2);
}