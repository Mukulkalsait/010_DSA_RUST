
use colored::*;
use rand::{Rng, RngExt};
use std::{cmp::Ordering, io};

pub fn game_main() {
    println!(
        "|==================> Please Select The Level {} vs {} <==================|",
        "'Easy'".green(),
        "'Hard'".red()
    );
    let mut level: String = String::new();
    io::stdin()
        .read_line(&mut level)
        .expect("⛔ Failure to read.");

    let mut easy = true;
    if level.trim() == "Hard" || level.trim() == "hard" {
        easy = false;
        println!("Hard Game Started Without Hints.")
    } else {
        println!("Easy Game Started With Hints.")
    }

    println!("|=================== Guseesing Game ==========================|");
    game_loop_start(easy);
    println!("|=================== End of the Game ==========================|");
}

fn secret_number() -> u32 {
    // Y: in this function we are returning only and only u32 value for that value to return we are not even adding variable.
    // G: In rust anythign which chanses its internel state must be mutable. RNG changes its internel state every time its called.
    let mut rng = rand::rng();
    rng.random_range(..1000)
}

fn guess_input() -> String {
    println!("| Guess The Number |");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("⛔Falure to read line !!!");
    guess
}

fn check_guess_number(n: String) -> u32 {
    //Y: older way let n: u32 = match n.trim().parse::<u32>() { Ok(num) => num, Err(_) => 0, };
    //
    //unwarp to default will give u32 = 0 | string = "" | bool = false | Vec = []  which are
    //there default valuse.

    let n: u32 = n.trim().parse::<u32>().unwrap_or_default();
    n
}

enum GuessResualt {
    Smaller, // Y: guess is smaler ans so on: weTried-> GuessIsSmaller->but then GuessIs part is common
    Greater,
    Equal,
}

fn match_guess(sec_num: u32, guess: u32) -> GuessResualt {
    match guess.cmp(&sec_num) {
        Ordering::Less => GuessResualt::Smaller,
        Ordering::Greater => GuessResualt::Greater,
        Ordering::Equal => GuessResualt::Equal,
    }
}

#[derive(Debug)] // Without this {:?} will not work in struct.
struct Range {
    lower_bound: u32,
    upper_bound: u32,
}

fn game_loop_start(easy: bool) {
    let sec_num = secret_number();
    let mut range_of_numbers: Range = Range {
        lower_bound: 1,
        upper_bound: 1000,
    }; // Y : Decleared : range -> we will lower it for more accurate op.

    loop {
        let guess_num = check_guess_number(guess_input());

        if guess_num == 0 {
            println!("🚫 Only Numbers between 1-1000 are allowed !!!");
            continue;
        }
        if easy {
            match match_guess(sec_num, guess_num) {
                GuessResualt::Smaller => smaller_guess(guess_num, &mut range_of_numbers),
                GuessResualt::Greater => greater_guess(guess_num, &mut range_of_numbers),
                GuessResualt::Equal => {
                    additional_functionality_additioon();
                    break;
                }
            }
        } else {
            match match_guess(sec_num, guess_num) {
                GuessResualt::Smaller => println!("{}", "Your guess is too small".red()),
                GuessResualt::Greater => println!("{}", "Your guess is too big".red()),
                GuessResualt::Equal => {
                    additional_functionality_additioon();
                    break;
                }
            }
        }
    }
}

fn smaller_guess(n: u32, range: &mut Range) {
    range.lower_bound = n;
    println!(
        "{}, {}, {}, {} to {}",
        "Your guess is too small".red(),
        "Please Input Higher Numbers".green(),
        "Updated Range is".blue(),
        range.lower_bound,
        range.upper_bound
    );
}
fn greater_guess(n: u32, range: &mut Range) {
    range.upper_bound = n;
    println!(
        "{}, {}, {}, {} to {}",
        "Your guess is too big".red(),
        "Please Input Lower Numbers".green(),
        "Updated Range is".blue(),
        range.lower_bound,
        range.upper_bound
    );
}

// fn update_after_guess() {}
// fn display_range() {}
// fn is_within_range() {}

fn additional_functionality_additioon() {
    println!("{}", "You Won !!! 💛💙🩶🩷🤍💖💛💙🩶🩷🤍💖".green());
    println!("next functionallitye is to add :
========================================================================================================
1. everyt time uer inout we are allready checking if it big or small , now we have to check how close the users guess is.
2. we will be needing 2 different mutable variables, a. for closest smaller guess and b. closest biger guess.
3. each time the guess is compaired with the exciting one guess if the guess is closer the newer will replace the older one.
4. and with the output we will be showing the result to choose in the newer 'shringked' range  with the help of a and b.
5. with this method we can increase the guess range drastically and add levels like easy, modaret, hard, and assested game or unassisted.
6. add what every you want to add just do it fast.
")
}
