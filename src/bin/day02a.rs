use std::collections::HashMap;
use std::io::{self, BufRead};
use std::iter::Iterator;

extern crate unicode_segmentation;
use unicode_segmentation::UnicodeSegmentation;

fn main() {
    // make a handle to the standard input of the current process
    let stdin = io::stdin();

    // read all of the input into a vector of strings (Vec<String>)
    let input = stdin.lock().lines().map(|line| line.unwrap()).collect();

    // feed the Vec<String> into the main logic and print the result
    println!("{:}", compute_checksum(input));
}

fn compute_checksum(input: Vec<String>) -> i32 {
    // store counts of box_ids with double letters and with triple letters
    let mut dubs = 0;
    let mut trips = 0;
    // loop over each box_id in the input
    for box_id in input.iter() {
        // check for double letters
        if has_multiples(2, &box_id) {
            dubs += 1;
        }
        // check for triple letters
        if has_multiples(3, &box_id) {
            trips += 1;
        }
    }
    // compute and return the checksum
    dubs * trips
}

fn has_multiples(count: usize, input: &str) -> bool {
    // store the number of occurances of each grapheme in the input
    let mut counts = HashMap::new();
    // loop over each grapheme in the input and update the counts hashmap
    for grapheme in UnicodeSegmentation::graphemes(input, true) {
        let g = counts.entry(grapheme).or_insert(0 as usize);
        *g += 1;
    }
    // return true if any graphemes have a count matching the desired count
    counts.values().any(|v| v == &count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            compute_checksum(vec![
                String::from("abcdef"),
                String::from("bababc"),
                String::from("abbcde"),
                String::from("abcccd"),
                String::from("aabcdd"),
                String::from("abcdee"),
                String::from("ababab"),
            ]),
            12
        );
    }

}
