use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub struct CurseWords {
    score: i8,

    // Length of password was greater than 15 chars
    good_length: bool,

    // Found a word that was found in password that was 4 or less chars in length
    has_word_less_4: i8,

    // Found a word that was found in password that was of length 5 - 8
    has_word_5_through_8: i8,

    // Found a word that was found in password that was longer than 8 characters
    has_word_greater_8: i8,

    // Makes sure the password is at least greater than 8 chars long
    need_to_run: bool,
}

impl CurseWords {
    fn new() -> CurseWords {
        CurseWords {
            score: 0,
            good_length: false,
            has_word_less_4: 0,
            has_word_5_through_8: 0,
            has_word_greater_8: 0,
            need_to_run: true,
        }
    }

    pub fn curse_score(input: String) -> CurseWords {
        let x: CurseWords = CurseWords::run_test(input);
        x
    }

    //TODO put all of the lengths as their own variable
    fn run_test(input: String) -> CurseWords {
        let mut test = CurseWords::new();

        // Checks the length of the input
        if input.len() >= 15 {
            test.good_length = true;
        } else if input.len() < 8 {
            test.need_to_run = false;
        }

        if test.need_to_run {
            let filename = "./shared/curse_words.txt";
            let file = File::open(filename).unwrap();
            let reader = BufReader::new(file);

            let input_size = input.len();

            for line in reader.lines() {
                let search = String::from(line.unwrap());
                let search_size = search.len();

                //` Checks to make sure that input can contain search word
                if input_size > search_size {
                    for i in 0..(input_size - search_size + 1) {
                        let mut temp = String::from(input.chars().nth(i).unwrap());
                        for j in 0..(search_size - 1) {
                            temp.push(input.chars().nth(j + i + 1).unwrap());
                        }
                        if temp == search {
                            if search_size <= 4 {
                                test.has_word_less_4 += 1;
                            } else if search_size <= 8 {
                                test.has_word_5_through_8 += 1;
                            } else {
                                test.has_word_greater_8 += 1;
                            }
                        }
                    }
                }
            }
        }

        test.score += test.has_word_less_4;
        test.score += test.has_word_5_through_8;
        test.score += test.has_word_greater_8;
        test
    }
}

#[cfg(test)]
mod curse_words {
    use super::*;

    #[test]
    fn curse_score_pass() {
        let x = String::from("This!hasNoB@dWordsInit");
        let result = CurseWords::curse_score(x);

        assert_eq!(0, result.score);
    }

    #[test]
    fn curse_score_pass_1() {
        let x = String::from("This!hasfuckNoB@dWordsInit");
        let result = CurseWords::curse_score(x);

        assert_eq!(1, result.score);
    }

    #[test]
    fn curse_score_returns_3_words_greater_than_8() {
        let x = String::from("fingerfuckingadsfudgepackera123@motherfucker ");
        let result = CurseWords::curse_score(x);

        assert_eq!(3, result.has_word_greater_8);
    }

    #[test]
    fn curse_score_returns_1_words_less_than_4() {
        let x = String::from("fahg1092uJanoasmkjlanob@*9ha ");
        let result = CurseWords::curse_score(x);

        assert_eq!(1, result.has_word_less_4);
    }
}
