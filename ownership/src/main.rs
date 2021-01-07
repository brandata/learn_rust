fn main() {
    let s =  "hello";
    let mut s2 = String::from("hello");

    s2.push_str(", world!");
    println!("{}",s2);
}
