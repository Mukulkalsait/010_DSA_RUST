
mod linear;
mod nested;
mod logarithmic;
mod nlog;

fn main() {
    // println!("----------| Linear |----------");
    // linear::run_linear();
    //
    // println!("----------| Nested |----------");
    // nested::run_nested();

    println!("----------| Logarithmic |----------");
    logarithmic::run_logarithmic();

    println!("----------| N Log N  |----------");
}
