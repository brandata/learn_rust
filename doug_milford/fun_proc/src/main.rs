#[allow(unused_variables)]

fn main() {
    let returned_data = some_function(2.2, 50);
    println!("returned_data is {}", returned_data);
    some_procedure(3.2, 1);

    some_str_procedure("test");

    let string_slice_var: &str = "Hello";
    some_str_procedure(string_slice_var);

    let string_var = String::from("String!");
    // need the & when using strings to str slice
    some_str_procedure(&string_var);
    
    some_string_procedure(&string_var);
    // cant use it again! unless there is the &
    some_string_procedure(&string_var);
}

fn some_string_procedure(param: &String) {
    println!("{}", param);
}

fn some_str_procedure(param: &str) {
    println!("{}",param);
}

fn some_procedure(param_a: f32, param_b: i8) {
    println!("I am in some procedure with a {} b {}", param_a, param_b);
}

fn some_function(param_a: f32, param_b: i128) -> f32 {
    println!("I am some function!");

    let return_var = 10.1 * param_a + param_b as f32;
    return_var // no semi colon means this is what is returned
}
