#[allow(unused_variables)]

fn main() {
    let some_tuple = (2, 3.5);
    println!("My data is {} {}", some_tuple.0, some_tuple.1);
    println!("my full tuple is {:?}", some_tuple);

    let new_tuple = (2, 3.5, "b".to_string(), 'c', (1.1, 2.2));
    let some_val = new_tuple.4 .1;

    let some_color = get_some_rbg();
    println!("Green is {}", some_color.1);
    
    let some_other_color: (u8, u8, u8, u8) = (0, 100, 150, 255);

    let empty_tuple = ();

    match some_color.2 {
        0..=200 => println!("blah"),
        _ => (),
    }
    
}

fn get_some_rbg() -> (u8, u8, u8) {
    // some logic
    (200, 100, 20)
}
