pub mod basic_checks;

use basic_checks::BasicPassword;

fn main() {
    let x = String::from("thisIs#agoodPassw0rd");
    BasicPassword::get_score(x);
}
