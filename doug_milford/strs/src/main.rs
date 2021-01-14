#[allow(unused_variables)]
fn main() {
    // usually immutable
    // often on stack, sometimes heap
    let example_str: &str = "Hello";

    // Stored on the heap
    // mutable
    let example_string: String = String::from("There");

    let string_from_str: String = example_str.to_string();
    // from string literal
    let string_from_str2: String = "hardcoded string".to_string();
    

    let string_from_hardcoded = String::from("some hardcode");
    let string_from_str_var = String::from(example_str);

    // points to String
    let str_from_string: &str = &example_string;

    // doesnt work
    //let test = "first" + "second";

    let combine_string_literals = ["first", "second"].concat();
    let combine_with_macro = format!("{} {}", "first", "second");

    let string_plus_str = example_string + example_str;

    let mut mut_string = String::new();
    mut_string.push_str(example_str);
    mut_string.push_str("hardcode");
    mut_string.push('m');
    // mut_string.push("m");
    println!("{}", mut_string);

    let a = String::from("a");
    let b = String::from("b");
    let combined = a + &b;
    println!("{}",combined);


    let str_from_substring: &str = &example_str[0..2];

    // get by index, can use to loop
    // returns an option
    let char_by_index = &example_str.chars().nth(1);

    match char_by_index {
        Some(c) => println!("Found a char{}", c),
        None => {}
    }

    if let Some(c) = example_str.chars().nth(2) {
        println!("Found a char {}", c);
    }
}
