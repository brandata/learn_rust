mod random_info;
use random_info::*;

#[allow(dead_code)]
#[derive(Debug)]
struct DougsData {
    some_bool: bool,
    some_float: f64,
    some_int: i32,
    random: RandomInfo,
}

impl RandomInfo {
    pub fn is_larger(&self, compare_to: i64) -> bool {
        self.some_int > compare_to
    }
}

impl SomeTrait for DougsData {
    fn is_valid(&self) -> bool {
        true
    }
}

impl Default for DougsData {
    fn default() -> Self {
        Self {
            some_bool: true,
            some_float: 10.2,
            some_int: 50,
            random: RandomInfo::new(true),
        }
    }
}

fn print_if_is_valid(check_me: &dyn SomeTrait) {
    if check_me.is_valid() {
        println!("Yay");
    }
}

#[allow(unused_variables)]
fn main() {

    let random_info_var = RandomInfo {
        call_count: 0,
        some_bool: true,
        some_int: 10,
    };

    let dougs_var = DougsData {
        some_bool: true,
        some_float: 1.2,
        some_int: 40,
        random: RandomInfo::new(true),
    };

    //let is_this_smaller = random_info_var.is_smaller(9);
    let is_larger = random_info_var.is_larger(29);
    let is_valid = random_info_var.is_valid();

    let default_doug = DougsData::default();

    //dougs_var.some_int = 100;

    //let dougs_var_2 = DougsData {
    //    some_int: 200,
    //    ..dougs_var
    //};

    print_if_is_valid(&random_info_var);
    print_if_is_valid(&dougs_var);

    println!("{:?}", dougs_var);

}
