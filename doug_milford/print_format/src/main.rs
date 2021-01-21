#[allow(unused_variables)]

#[derive(Debug)]
struct DougsData {
    pub a: i32,
    pub b: f32,
}

fn main() {
    // println prints then adds new line
    println!("Hello");
    // print doesnt add newline
    print!("There");
    println!("!");

    // adding data with {}, can index inputs
    // can also name inputs
    let more_data = 6.6;
    println!("My data is {1} and {0}",more_data, 6);

    let data = DougsData {
        a: 1,
        b: 1.1,
    };

    let other_data = DougsData {
        a: 2,
        b:2.2,
    };

    // need to implement display trait, or derive debug
    // add the # for pretty print
    println!("Dougs data: {1:#?} other data is {0:#?}", data, other_data);

    // can return formatted string to a variable:
    let formatted_string = format!("Dougs data: {1:#?} other data is {0:#?}", data, other_data);

    println!("{}", formatted_string);

    // ! -> declarative macro sign
    // # -> procedural macro sign
}
