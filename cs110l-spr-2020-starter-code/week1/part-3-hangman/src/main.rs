// Simple Hangman Program
// User gets five incorrect guesses
// Word chosen randomly from words.txt
// Inspiration from: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
// This assignment will introduce you to some fundamental syntax in Rust:
// - variable declaration
// - string manipulation
// - conditional statements
// - loops
// - vectors
// - files
// - user input
// We've tried to limit/hide Rust's quirks since we'll discuss those details
// more in depth in the coming lectures.
extern crate rand;
use rand::Rng;
use std::collections::HashMap;
use std::convert::TryInto;
use std::fs;
use std::io;
use std::io::Write;

const NUM_INCORRECT_GUESSES: u32 = 10;
const WORDS_PATH: &str = "words.txt";

fn pick_a_random_word() -> String {
    let file_string = fs::read_to_string(WORDS_PATH).expect("Unable to read file.");
    let words: Vec<&str> = file_string.split('\n').collect();
    String::from(words[rand::thread_rng().gen_range(0, words.len())].trim())
}

fn main() {
    let secret_word = pick_a_random_word();
    // Note: given what you know about Rust so far, it's easier to pull characters out of a
    // vector than it is to pull them out of a string. You can get the ith character of
    // secret_word by doing secret_word_chars[i].
    let secret_word_chars: Vec<char> = secret_word.chars().collect();
    // Uncomment for debugging:
    // println!("random word: {}", secret_word);

    // Your code here! :)
    hangman(&secret_word_chars)
    
}

fn hangman(secret_word_chars: &Vec<char>) {
    let mut pass = false;
    let mut guess_incorrect_num: u32 = 0;
    let mut guess_correct_chars = vec!['-'; secret_word_chars.len()];  //目前已经猜中
    let mut statistic = word_statistic(secret_word_chars);
    let mut cur_guessed_chars = Vec::new();
    while guess_incorrect_num < NUM_INCORRECT_GUESSES {
        println!("The word so far is {}", chars_to_string(&guess_correct_chars));
        println!(
            "You have guessed the following letters: {}",
             chars_to_string(&cur_guessed_chars)
        );
        println!(
            "You have {} guesses left",
            NUM_INCORRECT_GUESSES - guess_incorrect_num,
        );

        let ch = read_guess_char();

        if ch.is_alphabetic() {
            match is_char_in_word(ch, &mut statistic) {
                Index::Yes(idx) => {
                    guess_correct_chars[idx as usize] = ch;
                    if guess_correct_chars == *secret_word_chars {
                        pass = true;
                        break;
                    }
                }
                Index::No => {
                    guess_incorrect_num += 1;
                    println!("Sorry, that letter is not in the word")
                }
            }
            cur_guessed_chars.push(ch);
        } else {
            panic!("char is not alphabetic");
        }
        println!("------------ Round ends ---------------\n")
    }
    if pass {
        println!(
            "Congratulations you guessed the secret word: {}!",
            chars_to_string(&guess_correct_chars)
        )
    } else {
        println!("Sorry, you ran out of guesses!")
    }

}

fn chars_to_string(chars: &Vec<char>) -> String {
    chars.into_iter().collect()
}

fn read_guess_char() -> char {
    print!("Please guess a letter: ");
    io::stdout().flush().expect("Error flush stdout");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Error read lines");
    guess.as_bytes()[0] as char
}

struct CharInWord {
    num: u32,   // word中某个字符共有num个
    guessed_num: u32,   // 用户在猜字符过程中猜中了该字符多少次
    idxs: Vec<u32>,    // word中该字符的下标集合vec
}

enum Index {
    Yes(u32),
    No,
}

fn is_char_in_word(ch: char, statistic: &mut HashMap<char, CharInWord>) -> Index {
    match statistic.get_mut(&ch) {
        Some(ciw) => {
            let ch_idx;
            match ciw.idxs.get(ciw.guessed_num as usize) {
                Some(x) => ch_idx = *x,
                None => {
                    return Index::No;
                }
            }
            ciw.guessed_num += 1;
            Index::Yes(ch_idx)
        }
        None => Index::No
    }
}

fn word_statistic(secret_word_chars: &Vec<char>) -> HashMap<char, CharInWord> {
    let mut statistic = HashMap::new();
    for (idx, ch) in secret_word_chars.iter().enumerate() {
        let a = statistic.entry(*ch).or_insert(CharInWord {
            num: 0,
            guessed_num: 0,
            idxs: Vec::new(),
        });
        a.num += 1;
        a.idxs.push(idx.try_into().unwrap());
    }
    statistic
}

