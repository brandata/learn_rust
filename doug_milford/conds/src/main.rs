#[allow(unused_variables)]

fn main() {
    let some_bool = true;

    if some_bool == true {
        println!("Hit if branch");
    }

    // short hand
    if some_bool {
        println!("shorthand");
    }

    // ! is a not operator
    if !some_bool {
        println!("false");
    } else {
        println!("true");
    }

    let some_int = 30;
    let some_int2 = 20;

    if (some_bool == true || some_int > 10) && some_int2 == 100 {
        println!("true");
    } else {
        println!("false");
    }

    // inline
    let var_inline = if some_int == 9 {300} else {400};
    println!("{}", var_inline);


    match some_bool {
        true => {
            println!("Hit true");
        }
        false => {
            println!("Hit false");
        }
    }

    match some_int {
        0 => println!("hit 0"),
        1..=100 => println!("hit between 1 and 100"),
        _ => println!("hit else"),
    }
}
