#[allow(unused_variables)]

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

}

fn stack_procedure(param: f64) {
    println!("In stack_procedure with param {}", param);
}

fn heap_procedure(param: &Box<f64>) {
    println!("In heap_procedure with param {}", param);
}
