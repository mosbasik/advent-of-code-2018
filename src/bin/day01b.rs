use std::collections::HashSet;
use std::io::{self, BufRead};
use std::iter::Iterator;

fn main() {
    // make a handle to the standard input of the current process
    let stdin = io::stdin();

    // read all of the input as an iterator of Strings, then convert that into a
    // vector of i32's
    let input = stdin
        .lock()
        .lines()
        .map(|line| line.unwrap().parse::<i32>().unwrap())
        .collect();

    // feed the vector of i32's into the main logic and print the result
    println!("{:}", first_dupe(input));
}

fn first_dupe(input: Vec<i32>) -> i32 {
    // keep a set of the frequencies we've already seen; we'll find our first
    // duplicate frequency when we have our first hash collision
    let mut seen: HashSet<i32> = HashSet::new();

    // variable that tracks the current value of the (ever changing) frequency
    let mut freq: i32 = 0;

    // loop index
    let mut i: usize = 0;

    // insert the starting frequency into the set
    seen.insert(freq);

    // loop indefinitely (is broken out of when a result is found)
    loop {
        freq += input[i];
        let collision = !seen.insert(freq);
        if collision {
            return freq;
        }

        // update the loop index, using modulo division to keep going around
        i = (i + 1) % input.len();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(first_dupe(vec![1, -1]), 0);
    }

    #[test]
    fn test_2() {
        assert_eq!(first_dupe(vec![3, 3, 4, -2, -4]), 10);
    }

    #[test]
    fn test_3() {
        assert_eq!(first_dupe(vec![-6, 3, 8, 5, -6]), 5);
    }

    #[test]
    fn test_4() {
        assert_eq!(first_dupe(vec![7, 7, -2, -7, -4]), 14);
    }

}
