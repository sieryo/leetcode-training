use problem::roman_to_int;


pub mod problem;


fn run() -> () {
    let result = roman_to_int::roman_to_int("LVIII".to_string());

    println!("RESULT : {}", result);
}

fn main() {
    run();
}
