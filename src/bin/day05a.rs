use std::io::{self, BufRead};
use std::iter::Iterator;
use std::str::FromStr;

extern crate unicode_segmentation;
use unicode_segmentation::UnicodeSegmentation;

fn main() {
    // make a handle to the standard input of the current process
    let stdin = io::stdin();

    // read all of the input into a vector of strings (Vec<String>)
    let input = stdin.lock().lines().map(|line| line.unwrap()).collect();

    // feed the Vec<String> into the main logic and print the result
    println!("{:}", answer(input));
}

fn answer(input: Vec<String>) -> usize {
    match input.first() {
        // if the input contains at least one element, compute the contraction
        // of the first string and return its length
        Some(line) => contract(line).len(),

        // if the input contains no elements, panic
        None => panic!(),
    }
}

fn contract(input: &str) -> String {
    // convert input string into a vector of chars
    let mut chars: Vec<char> = UnicodeSegmentation::graphemes(input, true)
        .map(|g| char::from_str(g).unwrap())
        .collect();

    // initialize our two indices
    let mut rear: usize = 0;
    let mut front: usize = 1;

    // loop until front index runs off the end of the vector
    while front < chars.len() {
        // if the two indexed chars are the same letter (ignoring case)
        if chars[rear].eq_ignore_ascii_case(&chars[front]) {
            // and if the two indexed chars are different cases
            if chars[rear].is_ascii_lowercase() ^ chars[front].is_ascii_lowercase() {
                // remove both characters from the vector (being careful to
                // remove the one on the right first, because all the characters
                // to the right of a removal get shifted left by one)
                chars.remove(front);
                chars.remove(rear);

                // decrement the two indices by 1, as long as it won't make the
                // rear index run off the beginning of the vector. (In case the
                // characters before and after the removed characters need to be
                // removed.)
                if rear > 0 {
                    rear -= 1;
                    front -= 1;
                }

                // skip to the next iteration of the loop (to avoid incrementing
                // the indices)
                continue;
            }
        }

        // since no removal occured, increment the indices
        rear += 1;
        front += 1;
    }

    // collect the vector of chars into a String and return it
    chars.iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(contract("aA"), "");
    }

    #[test]
    fn test_2() {
        assert_eq!(contract("abBA"), "");
    }

    #[test]
    fn test_3() {
        assert_eq!(contract("abAB"), "abAB");
    }

    #[test]
    fn test_4() {
        assert_eq!(contract("aabAAB"), "aabAAB");
    }

    #[test]
    fn test_5() {
        assert_eq!(contract("dabAcCaCBAcCcaDA"), "dabCBAcaDA");
    }

}
