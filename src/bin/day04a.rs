use std::collections::HashMap;
use std::io::{self, BufRead};
use std::iter::Iterator;

extern crate chrono;
use chrono::NaiveDateTime;

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
    // create vector to hold our Log structs
    let mut logs = Vec::new();

    // parse a Log struct out of every line of the input
    for line in input.iter() {
        logs.push(parse_log(line));
    }

    // sort the Logs from earliest to latest by datetime
    logs.sort_unstable_by(|a, b| a.dt.cmp(&b.dt));

    // "indices" of a sort to track the current state as we walk through the logs
    let mut guard = None;
    let mut first_sleep_min: Option<usize> = None;
    let mut first_wake_min: Option<usize>;

    // a map of guard IDs to [a map of minutes to counts, where count is the
    // number of times this guard was asleep during this minute]
    let mut asleep = HashMap::new();

    // loop over every log entry, choosing a set of operations to perform based
    // on which one of three possible actions this log entry represents
    for log in logs.iter() {
        match log.action {
            // if a guard comes on duty, record his ID
            Action::Start => {
                guard = log.guard;
            }
            // if a guard goes to sleep, record the minute
            Action::Sleep => {
                first_sleep_min = Some(log.dt.format("%M").to_string().parse().unwrap());
            }
            // if a guard wakes up, record the minute.  then, for each minute
            // between the time they went to sleep and the time they woke up,
            // increment the appropriate minute counter in the "asleep" HashMap
            Action::Wake => {
                first_wake_min = Some(log.dt.format("%M").to_string().parse().unwrap());
                for min in first_sleep_min.unwrap()..first_wake_min.unwrap() {
                    let slot = asleep
                        .entry(guard.unwrap())
                        .or_insert(HashMap::new())
                        .entry(min)
                        .or_insert(0 as usize);
                    *slot += 1;
                }
                // unset the "going to sleep" minute for sanity
                first_sleep_min = None;
            }
        }
    }

    // find the guard who sleeps the most
    let guard = sleepiest_guard(&asleep);
    // find the minute of the night that guard sleeps the most
    let minute = sleepiest_minute(&asleep, guard);
    // multiply the two together and return the result
    guard * minute
}

fn sleepiest_guard(asleep: &HashMap<usize, HashMap<usize, usize>>) -> usize {
    asleep
        // make hashmap into an iterator over (k, v) tuples
        .iter()
        // convert to an iterator over (guard, total minutes asleep) tuples
        .map(|(guard, minute_counts)| (*guard, minute_counts.values().sum::<usize>()))
        // find the tuple with the highest total minutes asleep
        .max_by_key(|v| v.1)
        .unwrap()
        // return the guard ID of that tuple
        .0
}

fn sleepiest_minute(asleep: &HashMap<usize, HashMap<usize, usize>>, guard: usize) -> usize {
    match asleep.get(&guard) {
        Some(m) => *(m.iter().max_by_key(|x| x.1).unwrap().0),
        None => panic!(),
    }
}

#[derive(Debug)]
enum Action {
    Start,
    Sleep,
    Wake,
}

#[derive(Debug)]
struct Log {
    dt: NaiveDateTime,
    guard: Option<usize>,
    action: Action,
}

fn parse_log(input: &str) -> Log {
    // split input into (roughly) a datetime string and an action string
    let input_tokens = input.split(|c| c == '[' || c == ']').collect::<Vec<&str>>();

    // parse the datetime string into a NaiveDateTime object
    let dt = NaiveDateTime::parse_from_str(input_tokens[1], "%Y-%m-%d %H:%M").unwrap();

    // split the action string on the unicode word boundaries
    let action_tokens = UnicodeSegmentation::unicode_words(input_tokens[2]).collect::<Vec<&str>>();
    // use the first word to determine what Action is being taken
    let action = match action_tokens[0] {
        "Guard" => Action::Start,
        "falls" => Action::Sleep,
        "wakes" => Action::Wake,
        _ => panic!(),
    };
    // if the Action is that of a guard starting their shift, get their ID
    // number from the next word
    let guard = match action {
        Action::Start => Some(action_tokens[1].parse().unwrap()),
        _ => None,
    };

    // create and return the Log struct
    Log { dt, guard, action }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            answer(vec![
                String::from("[1518-11-01 00:05] falls asleep"),
                String::from("[1518-11-01 00:25] wakes up"),
                String::from("[1518-11-01 00:30] falls asleep"),
                String::from("[1518-11-01 00:55] wakes up"),
                String::from("[1518-11-01 00:00] Guard #10 begins shift"),
                String::from("[1518-11-01 23:58] Guard #99 begins shift"),
                String::from("[1518-11-02 00:40] falls asleep"),
                String::from("[1518-11-02 00:50] wakes up"),
                String::from("[1518-11-03 00:05] Guard #10 begins shift"),
                String::from("[1518-11-03 00:24] falls asleep"),
                String::from("[1518-11-03 00:29] wakes up"),
                String::from("[1518-11-04 00:02] Guard #99 begins shift"),
                String::from("[1518-11-04 00:36] falls asleep"),
                String::from("[1518-11-04 00:46] wakes up"),
                String::from("[1518-11-05 00:03] Guard #99 begins shift"),
                String::from("[1518-11-05 00:45] falls asleep"),
                String::from("[1518-11-05 00:55] wakes up"),
            ]),
            240
        );
    }

}
