#[allow(unused_variables)]

// constant
const DOUGS_CONSTANT: i64 = 200;

// static or global
static mut MY_STATIC_VARIABLE: i32 = 10;

fn main() {
    let some_i32: i32 = 10;
    let some_f64: f64 = 20.3;

    // casting must be explicit:
    // data lost (the decimal)
    let combined = some_i32 + some_f64 as i32;
    println!("{}", combined);

    // shadowing: define two different variables with the same
    // name in different scopes
    let var_a: i32 = 10;
    { //inner scope
        println!("Inner {}", var_a); // 10
        // create same variable in this scope
        let var_a: f32 = 20.42;
        println!("Inner {}", var_a); // 20.42
    }
    println!("Inner {}", var_a); // 10    

    // constants: defined above
    // during compile time, the constant value in inserted
    // whereever it is used, it is not contained in memory
    // some built in:
    let circle_pi = std::f32::consts::PI;
    println!("pi: {}", circle_pi);

    // static or global variables
    // they are mutable. Can only change them in unsafe scopes:
    // even printing them must be done in unsafe
    unsafe {
        MY_STATIC_VARIABLE = 20;
        println!("{}", MY_STATIC_VARIABLE);
    }
}
