fn main() {
   let my_string = String::from("hello world");
   let word = first_word_slice(&my_string[..]);
   let my_string_literal = "hello world";

}

// First way - bad
fn first_word(s: &String) -> {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

// Slice way
fn first_word_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter.enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
