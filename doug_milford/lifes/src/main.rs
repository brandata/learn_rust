// LIFETIMES
// -lifetimes enforce a piece of memory is still valid for 
// a reference
// - lifetimes ensure memory does not get cleaned up before
// a reference can use it !!!!!

const SOME_INT: i32 = 40;// lifetime of full program

fn main() {
    let a;
    {// artificial inner scope
        let b = String::from("Hi");
        // below, ownership is changed from b to a, so it will
        // be present after this scope.
        a = b;
        
        // if a becomes a reference to be, there will be a lifetime
        // error
        // a = &b;
    }// b cleaned up here

    println!("{}",a);

    // scope of input is the same as the scope ofoutput
    let some_int_var = 10;
    let some_int_var2 = 20;
    let result_ref = get_int_ref(&some_int_var, some_int_var2);
    println!("{}", result_ref);
}

// This isnt valid. Rust knows that since there are no params
// the returned reference must come within the function itself
// meaning once the function returns it won't be referencing
// anyting anymore
//fn get_int_ref() -> &i32 {
//    let a = 1;
//    &a;
//}

// This works because the scope providing the reference is the
// same exat scope that will be receiving the result output
// - lifetimes are defined with <'a>
// - then can apply lifetime to input or output
fn get_int_ref<'a>(param_1: &'a i32, param_2: i32) -> &'a i32 {
    println!("{}", param_2);
    param_1
}

// Here we have two different lifetimes for each param,but are
// returning lifetime a. Because, depending on the params,
// lifetime a or b could be returned, the compiler doesnt allow
// this. There is ambiguity regarding which will be returned
// - This can be fixed by telling the compiler that b has to live at least as long as a 'b: 'a
// OR just give both variables the same lifetime because they
// come from the same scope
fn get_int_ref2<'a, 'b: 'a>(param_1: &'a i32, param_2: &'b i32) -> &'a i32 {
    if param_1 > param_2 {
        param_1
    } else {
        param_2
    }
}

fn get_str_ref<'a>(param_1: &'a str, param_2: &'a str) -> &'a str {
    if param_1 > param_2 {
        param_1
    } else {
        param_2
    }
}

// Lifetimes dont apply because there are no references
#[allow(dead_code)]
fn test_1(param_1: Vec<f64>) -> Vec<f64> {
    param_1
}

// Lifetimes arent an issue because there is no reference output
#[allow(dead_code)]
fn test_2<'a>(param_1: &'a Vec<f64>) -> Vec<f64> {
    param_1.clone()
}

// Lifetimes dont apply because there are no reference inputs
#[allow(dead_code)]
fn test_3<'a>(param_1: Vec<f64>) -> &'a Vec<f64> {
    &param_1
}

#[allow(dead_code)]
fn test_4<'a>(param_1: i32, param_2: &'a str, param_3: &'a str, param_4: f64) -> &'a str {
    if param_1 == 7 && param_4 > 10. {
        param_2
    } else {
        param_3
    }
}

