// if want one with other type, cant reuse nae
//struct Point {
//    y: i32,
//    x: i32,
//}

// Generic - abstract/placeholder types
// T is a placerholder, can use anything but T is convention
// Can use different letters if you want different input types
struct Point<T> {
    x: T,
    y: T,
}

// can use normal types too
struct Data<T> {
    x: i32,
    y: T, 
    Z: T,
    some_char: char,
}

enum SomeEnum<T> {
    OptionA(T),
    OptionB(T),
    OptionC,
}

// custom trait
trait SomeCustomTrait {
    fn blah_blah(&self, a: &str, b:&str) -> String;
}

#[derive(Debug)]
struct DougsStruct {
    something: i32,
}

impl SomeCustomTrait for DougsStruct {
    fn blah_blah(&self, a: &str, b:&str) -> String {
        self.something.to_string() + " - " + a + " - " + b
    }
}

//implementing a custom trait
fn do_this<T>(some_var: &T) -> String
where T: SomeCustomTrait + std::fmt::Debug {
    println!("{:?}", some_var);
    some_var.blah_blah("first","second")
}
//another way to implement trait, but can only implement one per
//variable
//fn do_this2(some_var: &dyn SomeCustomTrait) -> String { 
//    println!("{:?}", some_var);
//    some_var.blah_blah("first","second")
//}

impl SomeCustomTrait for i32 {
    fn blah_blah(&self, a: &str, b: &str) -> String {
        "i32".to_string() + " - " + a + " - " + b
    }
}
////////////////////////

#[allow(dead_code)]
struct BransStruct<T, U> {
    brans_t: T,
    brans_u: U,
}

impl<T,U> BransStruct<T, U>
where T: std::fmt::Debug,
      U: std::fmt::Debug {
    fn log_something(&self) {
        println!("{:?} {:?}", self.brans_t, self.brans_u);
    }
}

fn main() {

    let test = BransStruct {
        brans_t: 5.6,
        brans_u: vec![1, 2, 3],
    };

    test.log_something();


    /////////////////////////// 
    let a = Point {x:100, y:-1};
    println!("{} {}", a.x, a.y);

    let b = Point {x: 1.6, y: 20.4};
    println!("{} {}", b.x, b.y);

    let some_data = SomeEnum::OptionA(35.2);

    match some_data {
        SomeEnum::OptionA(a) => println!("OptionA {}", a),
        SomeEnum::OptionB(b) => println!("OptionB {}", b),
        SomeEnum::OptionC => println!("OptionC"),
    }

    let some_data2 = SomeEnum::OptionB('b');
    let some_data3 = SomeEnum::OptionA(vec![1, 2, 3]);

    let a = dougs_fn2(4, 6);
    println!("a has {}", a);

    let test = DougsStruct { something: 2000 };
    let result = do_this(&test);

    let testi32 = 18;
    let result2 = do_this(&testi32);
}

// Adds a constraint after T to ensure we can add
fn dougs_fn<T: std::ops::Add<Output=T> + std::ops::Sub<Output=T>>(input_a: T, input_b: T) -> T { 
    input_a + input_b 
}

// Adds a constraint after T to ensure we can add
fn dougs_fn2<T>(input_a: T, input_b: T) -> T 
where T: std::ops::Add<Output=T> + std::ops::Sub<Output=T> + std::fmt::Debug { 
    println!("input has {:?}", input_a);
    input_a + input_b 
}
