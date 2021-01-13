// Warning overrides
#[allow(unused_variables)]
#[allow(unused_assignments)]

fn main() {
    
    // boolean
    let mut some_data: bool = true;
    some_data = false;

    // integers
    let some_data: i8 = 10; // from -128 to 127

    // max and mins
    println!("Min i8 is {}", std::i8::MIN);
    println!("Max i8 is {}", std::i8::MAX);

    // let overflow_test = some_data + 120;

    // panics unless program is run in release mode
    // println!("{}", overflow_test);

    let new_data: u8 = 59; // from 0 to 255
    println!("Max i128 is {}", std::i128::MAX);

    let some_isize: isize = 10; // depends on computer-32 or 64bit
    let some_usize: usize = 24; // same here

    // floats
    let some_float: f32 = 10.; // needs to have the decimal point
    // default is 64
    
    // chars
    let some_char: char = 'a'; // stores more than just ascii- emoji
}
