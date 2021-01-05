fn main() {
    println!("Hello, world!");
    let x = 4_0u8;
    println!("{}",x);

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // Tuples and pattern matching
    let (a, b, c) = tup;
    println!("The value of a is {}", a);
    println!("The value of b is {}", tup.1);


    //Arrays
    let a = [1, 2, 3, 4, 5];
    println!("The first element of a is {}", a[0]);
}
