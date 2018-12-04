use std::collections::{HashMap, HashSet};
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
    println!("{:}", first_uncontested_claim(input).unwrap());
}

fn first_uncontested_claim(input: Vec<String>) -> Option<usize> {
    // keep track of the claims made to each square inch of the cloth
    let mut cloth = HashMap::new();

    // keep track of the extant claim IDs
    let mut claim_ids = HashSet::new();

    // loop over each claim string
    for line in input.iter() {
        // parse the claim string into a claim struct for easier access
        let claim = parse_claim(line);

        // update the set of all claim IDs
        claim_ids.insert(claim.id);

        // compute the first and last columns and rows of this claim
        let col_first = claim.left_margin + 1;
        let col_last = claim.left_margin + claim.width;
        let row_first = claim.top_margin + 1;
        let row_last = claim.top_margin + claim.height;

        // for every square inch of this claim
        for col in col_first..col_last + 1 {
            for row in row_first..row_last + 1 {
                // tag this square inch with this claim's ID
                cloth.entry((col, row)).or_insert(Vec::new()).push(claim.id);
            }
        }
    }

    // discard all square inches that are tagged with fewer than 2 claim IDs
    cloth.retain(|_, v| v.len() > 1);

    // loop over contested square inches and remove all claim IDs appearing in
    // them from the set of claim IDs (leaving behind only claim IDs that are
    // *not* contested)
    for contest in cloth.values() {
        for claim_id in contest.iter() {
            claim_ids.remove(claim_id);
        }
    }

    // loop over the set of uncontested claim IDs and return the first one
    for claim_id in claim_ids.iter() {
        return Some(*claim_id);
    }

    // if the set of uncontested claim IDs is empty, return None
    None
}

#[derive(Debug)]
struct Claim {
    id: usize,
    left_margin: usize,
    top_margin: usize,
    width: usize,
    height: usize,
}

fn parse_claim(input: &str) -> Claim {
    // split the input string on unicode word boundaries
    let words = UnicodeSegmentation::unicode_words(input).collect::<Vec<&str>>();

    // the first word is the claim id
    let id = words[0].parse().unwrap();

    // the second word is the left and top margins with a "," in between
    let margins = words[1]
        .split(",")
        .map(|m| m.parse().unwrap())
        .collect::<Vec<usize>>();
    let left_margin = margins[0];
    let top_margin = margins[1];

    // the third word is the width and height values with a "x" in between
    let dimensions = words[2]
        .split("x")
        .map(|m| m.parse().unwrap())
        .collect::<Vec<usize>>();
    let width = dimensions[0];
    let height = dimensions[1];

    // create and return a claim struct
    Claim {
        id,
        left_margin,
        top_margin,
        width,
        height,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            first_uncontested_claim(vec![
                String::from("#1 @ 1,3: 4x4"),
                String::from("#2 @ 3,1: 4x4"),
                String::from("#3 @ 5,5: 2x2"),
            ]).unwrap(),
            3
        );
    }

}
