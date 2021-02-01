#[derive(Debug)]
pub struct BasicPassword {
    score: i8,

    good_length: bool,

    uses_uppercase: bool,

    uses_lowercase: bool,

    uses_symbol: bool,

    uses_number: bool,
}

impl BasicPassword {
    fn new() -> BasicPassword {
        BasicPassword {
            score: 0,
            good_length: false,
            uses_lowercase: false,
            uses_uppercase: false,
            uses_symbol: false,
            uses_number: false,
        }
    }
    pub fn get_score(input: String) {
        let x = BasicPassword::run_test(input);

        println!("Your Score is {} / 5", x.score);
    }

    fn run_test(input: String) -> BasicPassword {
        let lower_case_characters = String::from("abcdefghijklmnopqrstuvwxyz");
        let upper_case_characters = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
        let numbers = String::from("1234567890");
        let symbols = String::from("`~:;,<>./?'\"]}[{=+-_)(*&^%$#@!");

        let mut test = BasicPassword::new();
        if input.len() >= 15 {
            test.good_length = true;
        }
        for i in input.chars() {
            for j in lower_case_characters.chars() {
                if i == j {
                    test.uses_lowercase = true;
                }
            }
            for j in upper_case_characters.chars() {
                if i == j {
                    test.uses_uppercase = true;
                }
            }
            for j in numbers.chars() {
                if i == j {
                    test.uses_number = true;
                }
            }
            for j in symbols.chars() {
                if i == j {
                    test.uses_symbol = true;
                }
            }

            if test.uses_lowercase && test.uses_uppercase && test.uses_number && test.uses_symbol {
                break;
            }
        }

        if test.uses_lowercase == true {
            test.score += 1;
        }
        if test.uses_uppercase == true {
            test.score += 1;
        }
        if test.uses_number == true {
            test.score += 1;
        }
        if test.uses_symbol == true {
            test.score += 1;
        }
        if test.good_length == true {
            test.score += 1;
        }
        test
    }
}
