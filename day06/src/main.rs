use std::collections::HashSet;

struct Datastream {
    raw: &'static str,
    start_of_packet: usize,
    start_of_message: usize,
}

impl Datastream {
    fn new(input: &'static str) -> Self {
        use itertools::Itertools;
        let Some(first_unique_tuple) = input
            .chars()
            .into_iter()
            .enumerate()
            .tuple_windows()
            .find(|(a, b, c, d)| HashSet::from([a.1, b.1, c.1, d.1]).len() == 4)
            else {
                panic!("No valid start found");
            };
        let Some((start_of_message, _)) = input
            .chars()
            .collect::<Vec<_>>()
            .windows(14)
            .find_position(|w| HashSet::<_>::from_iter(w.iter()).len() == 14)
            else {
            panic!("No valid start found");
        };

        Self {
            raw: input,
            start_of_packet: first_unique_tuple.3 .0 + 1,
            start_of_message: start_of_message + 14,
        }
    }
}

fn main() {
    let input = include_str!("input.txt");
    let datastream = Datastream::new(input);
    let start_index = datastream.start_of_packet;
    let start_of_message = datastream.start_of_message;

    println!(
        "The datastream starts at index {} and the message at {}!",
        start_index, start_of_message
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seeking_packet() {
        assert_eq!(
            Datastream::new("bvwbjplbgvbhsrlpgdmjqwftvncz").start_of_packet,
            5
        );
        assert_eq!(
            Datastream::new("nppdvjthqldpwncqszvftbrmjlhg").start_of_packet,
            6
        );
        assert_eq!(
            Datastream::new("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg").start_of_packet,
            10
        );
        assert_eq!(
            Datastream::new("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw").start_of_packet,
            11
        );
    }

    #[test]
    fn test_seeking_message() {
        assert_eq!(
            Datastream::new("mjqjpqmgbljsphdztnvjfqwrcgsmlb").start_of_message,
            19
        );
        assert_eq!(
            Datastream::new("bvwbjplbgvbhsrlpgdmjqwftvncz").start_of_message,
            23
        );
        assert_eq!(
            Datastream::new("nppdvjthqldpwncqszvftbrmjlhg").start_of_message,
            23
        );
        assert_eq!(
            Datastream::new("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg").start_of_message,
            29
        );
    }
}
