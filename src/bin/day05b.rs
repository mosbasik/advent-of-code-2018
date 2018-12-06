use std::collections::{HashMap, HashSet};
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
    // the problem indicates we will only get one line of input, so perform a
    // sanity check and then save that line outside of the input vector
    assert_eq!(input.len(), 1);
    let line = &input[0];

    // get the list of unique polymers found in this line
    let removal_candidates = get_polymers(&line);

    // make an empty map (k: polymer, v: line-excluding-that-polymer)
    let mut excluded: HashMap<char, String> = HashMap::new();

    // for each unique polymer, add an entry to the excluded map
    for polymer in removal_candidates.iter() {
        excluded.insert(*polymer, exclude_polymer(&line, *polymer));
    }

    // make an empty map (k: polymer, v: contracted-line-excluding-that-polymer)
    let mut contracted = HashMap::new();

    // for each unique polymer, add an entry to the contracted map
    for (polymer, excluded_line) in excluded.iter() {
        contracted.insert(polymer, contract(&excluded_line));
    }

    // return the polymer that results in the smallest contracted line
    contracted
        .iter()
        .map(|(p, c)| (p, c.len()))
        .min_by_key(|x| x.1)
        .unwrap()
        .1
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
                // character before removal reacts with the character after the
                // removal.)
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

fn get_polymers(input: &str) -> HashSet<char> {
    // get iterator of graphemes
    UnicodeSegmentation::graphemes(input, true)
        // convert to iterator of chars
        .map(|g| char::from_str(g).unwrap())
        // convert to iterator of lowercase chars
        .map(|c| c.to_ascii_lowercase())
        // collect into a HashSet<char> (thus removing duplicates) and return
        .collect()
}

fn exclude_polymer(input: &str, polymer: char) -> String {
    // sanity check to limit input space
    assert!(polymer.is_ascii_lowercase());

    // get an uppercase version of the polymer of interest
    let polymer_uppercase = polymer.to_ascii_uppercase();

    // convert input string to vector of chars
    let mut chars: Vec<char> = UnicodeSegmentation::graphemes(input, true)
        .map(|g| char::from_str(g).unwrap())
        .collect();

    // initialize loop index
    let mut i = 0;

    // loop over all characters in vector
    while i < chars.len() {
        // if the indexed char matches either polarity of the polymer in question
        if (chars[i] == polymer) || (chars[i] == polymer_uppercase) {
            // remove that char from the vector
            chars.remove(i);
            continue;
        }
        // otherwise, increment the index
        i += 1;
    }

    // collect the vector of chars into a string and return it
    chars.iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(exclude_polymer("dabAcCaCBAcCcaDA", 'a'), "dbcCCBcCcD");
    }

    #[test]
    fn test_2() {
        assert_eq!(exclude_polymer("dabAcCaCBAcCcaDA", 'b'), "daAcCaCAcCcaDA");
    }

    #[test]
    fn test_3() {
        assert_eq!(exclude_polymer("dabAcCaCBAcCcaDA", 'c'), "dabAaBAaDA");
    }

    #[test]
    fn test_4() {
        assert_eq!(exclude_polymer("dabAcCaCBAcCcaDA", 'd'), "abAcCaCBAcCcaA");
    }

    #[test]
    fn test_5() {
        assert_eq!(contract("dabAcCaCBAcCcaDA"), "dabCBAcaDA");
    }

    #[test]
    fn test_6() {
        let mut expected = HashSet::new();
        expected.insert('a');
        expected.insert('b');
        expected.insert('c');
        expected.insert('d');
        assert_eq!(get_polymers("dabAcCaCBAcCcaDA"), expected);
    }

    #[test]
    fn test_7() {
        assert_eq!(answer(vec![String::from("dabAcCaCBAcCcaDA")]), 4);
    }

}
