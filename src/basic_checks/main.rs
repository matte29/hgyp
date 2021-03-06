#[derive(Debug)]
pub struct BasicPassword {
    score: i8,

    good_length: bool,

    uses_uppercase: bool,

    uses_lowercase: bool,

    uses_symbol: bool,

    uses_number: bool,

    need_to_run: bool,
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
            need_to_run: true,
        }
    }
    pub fn get_score(input: String) -> i8 {
        let x = BasicPassword::run_test(input);
        let return_value = x.score;
        return_value
    }

    fn run_test(input: String) -> BasicPassword {
        let lower_case_characters = String::from("abcdefghijklmnopqrstuvwxyz");
        let upper_case_characters = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
        let numbers = String::from("1234567890");
        let symbols = String::from("`~:;,<>./?'\"]}[{=+-_)(*&^%$#@!");

        let mut test = BasicPassword::new();
        if input.len() >= 15 {
            test.good_length = true;
        } else if input.len() < 8 {
            test.need_to_run = false;
        }

        if test.need_to_run {
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

                if test.uses_lowercase
                    && test.uses_uppercase
                    && test.uses_number
                    && test.uses_symbol
                {
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
        }
        test
    }
}

//`------------------
//` name: basic_passwords
//`
//` @param: none
//`
//` Purpose: Tests get_score, and through using that tests run_test
//`
//` Return's: /*Return Value*/
//`------------------
#[cfg(test)]
mod basic_passwords {
    use super::*;
    #[test]
    fn get_score_pass_5_out_of_5() {
        let x = String::from("thisIs#agoodPassw0rd");
        let result = BasicPassword::get_score(x);

        assert_eq!(5, result);
    }

    #[test]
    fn get_score_pass_1_out_of_5() {
        let x = String::from("thisisbad");
        let result = BasicPassword::get_score(x);

        assert_eq!(1, result);
    }

    #[test]
    fn get_score_0_out_of_5() {
        let x = String::from("Jonny");
        let result = BasicPassword::get_score(x);

        assert_eq!(0, result);
    }
}
