pub mod basic_checks;

//use basic_checks::BasicPassword;

pub mod word_checks;

use word_checks::CurseWords;

fn main() {
    let x = String::from("thisHas24NobadWords");
    let result: CurseWords = CurseWords::curse_score(x);
    println!("{:?}", result);
}
