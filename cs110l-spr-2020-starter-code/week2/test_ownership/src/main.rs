

fn drip_drop() -> String {
    let s = String::from("hello world!");
    return s;
}

fn main() {
    let s1 = String::from("hello");
    let mut v = Vec::new();
    v.push(s1);
    let s2 = &v[0];
    println!("{}", s2);

}
