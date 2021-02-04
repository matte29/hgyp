use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub struct TopWords {
    //` Score for Top Words tests
    score: i8,

    has_word_less_4: i8,

    has_word_5_through_8: i8,

    has_word_greater_8: i8,

    need_to_run: bool,
}

impl TopWords {
    fn new() -> TopWords {
        TopWords {
            score: 0,
            has_word_less_4: 0,
            has_word_5_through_8: 0,
            has_word_greater_8: 0,
            need_to_run: true,
        }
    }

    pub fn top_words_score(input: String) -> TopWords {
        let x: TopWords = TopWords::run_top_test(input);

        //` Returns x: TopWords
        x
    }

    fn run_top_test(input: String) -> TopWords {
        let mut test = TopWords::new();

        let input_size = input.len();

        //` Check and Store the length of the input
        if input_size < 8 {
            test.need_to_run = false;
        }

        if test.need_to_run {
            let filename = "./shared/most_common_words.txt";
            let file = File::open(filename).unwrap();
            let reader = BufReader::new(file);

            for line in reader.lines() {
                let search = String::from(line.unwrap());
                let search_size = search.len();

                //` Checks to make sure the input can contain the search word.
                if input_size > search_size {
                    for i in 0..(input_size - search_size + 1) {
                        //` temp holds the
                        let mut temp = String::from(input.chars().nth(i).unwrap());
                        for j in 0..(search_size - 1) {
                            temp.push(input.chars().nth(j + i + 1).unwrap());
                        }

                        //`
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
mod top_words {
    use super::*;

    #[test]
    fn top_words_score_pass() {
        let x = String::from("n0C08w0rdswh2jskr");
        let result = TopWords::top_words_score(x);

        assert_eq!(0, result.score);
    }

    #[test]
    fn top_words_score_pass_1() {
        let x = String::from("tyahisna309fasdnfj4a90gn!");
        let result = TopWords::top_words_score(x);

        assert_eq!(1, result.score);
    }

    #[test]
    fn top_words_score_returns_2_words_between_5_and_8() {
        let x = String::from("aniuhelloanasquad");
        let result = TopWords::top_words_score(x);

        assert_eq!(2, result.has_word_5_through_8);
    }

    #[test]
    fn top_words_score_returns_1_words_less_than_4() {
        let x = String::from("anfslh3294j#(0ya)");
        let result = TopWords::top_words_score(x);

        assert_eq!(1, result.has_word_less_4);
    }
}
