
mod modules;
mod linear;
mod nested;
mod logarithmic;
mod nlogn;
mod game;


fn main() {
    println!("Hello and welcome to DSA in rust - learning...");

    // let mut input= String::from("");

    // select_what_to_learn();

    game::game::game_main();

}

fn select_what_to_learn(){


    // println!("----------| Linear |----------");
    // linear::run_linear();
    //
    // println!("----------| Nested |----------");
    // nested::run_nested();
    //
    // println!("----------| Logarithmic |----------");
    // logarithmic::run_logarithmic();

    // println!("----------| N Log N  |----------");
    // nlogn::run_n_log_n();
    
    println!("----------|  matrix_sum  |----------");
    // modules::matrix_sum::matrics_sum_main();

}

