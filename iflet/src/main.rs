fn main() {
    // Want to only process the Some(3) variant
    // Lots of boiler plate
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => ()
    }

    // if let way of accomplishing the same
    if let Some(3) = some_u8_value {
        println!("three")
    }
}
