#[allow(unused_variables)]

#[derive(Debug, Clone, Copy)]
struct DougsStruct {
    a: i32,
    b: f64,
}

fn main() {
    // STACK
    // - fast memory creation and retrieval
    // - memory is automatically recaptured when variables go out
    // - memory known at compile time
    // - fixed size
    // - default in rust
    //
    //
    //  
    // Example of common problem/misunderstanding:
    //    let var_a = String::from("Howdy");
    //    let var_b = var_a;
    //
    //    println!("{}", var_a); <- compile error

    // stack variables:
    let stack_i8: i8 = 10;
    let stack_f32: f32 = 52.;
    let stack_bool: bool = true;
    let stack_char: char = 'a';

    if stack_i8 == 3 {
        let inside_scope = 9;
        println!("{}", inside_scope);
    }// scope ends here!
    // inside_scope is gone here

    // HEAP
    // - flexibility
    // - memory can grow in size (vector, hashmap, string, etc)
    // - runtime performance cost (speed)
    // - memory can live beyond the scope that created it
    // - automatically recaptured when last owner goes out of scope
   // example heap variables 
    let heap_vector: Vec<i8> = Vec::new();
    let heap_string: String = String::from("hi");
    let heap_i8: Box<i8> = Box::new(30);

    // OK
    // this works because stack copies are cheap, so each
    // variable owns different memory
    let stack_i8_2 = stack_i8;
    println!("{}", stack_i8);
    println!("{}", stack_i8_2);

    // not OK
    // every piece of memory has an owner
    // heap_i8 is the original owner of the memory
    // creating heap_i8_2 keeps allocated memory intact but transfers ownership to heap_i8_2
    // after this, heap_i8 points to nothing!
    // One way to get around this is to borrow ownsership
    //  as a reference - &
    // Another way is to clone it (copies it so there are 2)
    let heap_i8_2 = heap_i8.clone();// &heap_i8;
    println!("{}", heap_i8);
    println!("{}", heap_i8_2);
    
    // EXAMPLES ///////////////// 

    let stack_f64: f64 = 1.;
    let heap_f64: Box<f64> = Box::new(2.);

    // - can call and still use stack variable afterwards
    // - because its a stack variable, it gets copied for the para
    // - if param is mutable, the original variable remains unchanged 
    stack_procedure(stack_f64);
    println!("In main stack {}", stack_f64);

    // - compile error on the print statement
    // - this is because ownership of memory associated with
    // heap64 gets transferred to param. Then after the scope
    // in heap_procedure ends, it gets cleaned up so it doesnt
    // exist after returning.
    // - to get around this, use borrowing
    // - function borrows memory using &
    // - when you see & it means the memory ownership will be
    // borrowed for the procedure call
    heap_procedure(&heap_f64);
    println!("In main stack {}", heap_f64);


    //////////// STRINGS ////////////////////////
    // They are always on the heap
    let some_string: String = String::from("Hello");
    // - str slice not locked to start or heap
    // - pointer to some one elses memory location, string 
    // slice is borrowing it. It borrows and points to original
    // memory
    let some_str: &str = "There";

    // after this function is called, some_string will be unavailable unless it is borrowed with &
    some_procedure(&some_string, some_str);


    // - heap data can only have 1 owner at a time, but can
    // have many references if the variable is mutable or 
    // doesnt change
    let var_a = String::from("Hello!");
    let var_b = &var_a;
    let var_c = &var_a;
    // var_a maintains ownership, b and c are read only 
    println!("{} {} {}", var_a, var_b, var_c);

    let var_d = String::from("Hello");
    let var_e = String::from("there");

    let mass_data: Vec<&String> = vec![&var_d, &var_e];

    println!("{}", heavy_calcs(&mass_data));

    println!("{} {}", var_d, var_e);

    // structs
    // dougs struct contains stack variables
    let var_1 = DougsStruct { a: 9, b: 10. };
    // there is no copy of the data to param_a like with normal
    // stack variables (struct could be huge)
    some_struct_proc(&var_1);
    // this wont work!
    //
    // need to use a borrowed reference or clone it or copy
    println!("{:?}", var_1);

}

fn heavy_calcs(_param: &Vec<&String>) -> i64 {
    // some heavy duty cales
    
    10
}

fn stack_procedure(param: f64) {
    println!("In stack_procedure with param {}", param);
}

fn heap_procedure(param: &Box<f64>) {
    println!("In heap_procedure with param {}", param);
}

fn some_procedure(param_a: &String, param_b: &str) {
    println!("{} {}", param_a, param_b);
}

fn some_struct_proc(param_a: &DougsStruct) {
    println!("{:?}", param_a);
}
