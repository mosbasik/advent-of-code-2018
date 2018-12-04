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
    println!("{:}", find_pair(input).unwrap());
}

fn find_pair(input: Vec<String>) -> Option<String> {
    // loop over every pair of box_id's :(
    for (i, box_id_a) in input.iter().enumerate() {
        for box_id_b in input.iter().skip(i + 1) {
            // if a pair of box_id's only differs by one character, return all
            // the common characters
            match off_by_one(&box_id_a, &box_id_b) {
                Some(common_letters) => return Some(common_letters),
                None => continue,
            }
        }
    }
    None
}

fn off_by_one(a: &str, b: &str) -> Option<String> {
    // check lengths of box_ids bc current logic would break if they're unequal
    assert_eq!(a.len(), b.len());

    // zip the pair of box_ids into an iterator of tuples that's easy to loop over
    let a_graphemes = UnicodeSegmentation::graphemes(a, true);
    let b_graphemes = UnicodeSegmentation::graphemes(b, true);
    let zipped_graphemes = a_graphemes.zip(b_graphemes);

    // store characters that occur in the same place in both box_ids in this
    // vector to be returned if necessary
    let mut common_graphemes = vec![];

    // loop over pairs of characters and populate the common_graphemes vector
    for (ga, gb) in zipped_graphemes {
        if ga == gb {
            common_graphemes.push(ga);
        }
    }

    // if all but exactly one of the characters match, return positively,
    // otherwise return negatively
    if common_graphemes.len() == a.len() - 1 {
        return Some(common_graphemes.into_iter().collect());
    } else {
        return None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            find_pair(vec![
                String::from("abcde"),
                String::from("fghij"),
                String::from("klmno"),
                String::from("pqrst"),
                String::from("fguij"),
                String::from("axcye"),
                String::from("wvxyz"),
            ]).unwrap(),
            String::from("fgij")
        );
    }

}
