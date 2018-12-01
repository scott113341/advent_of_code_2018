use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");
    let frequencies = frequencies_from_string(&input.to_string());

    println!("{}", final_frequency(&frequencies));
    println!("{}", first_repeated_frequency(&frequencies));
}

pub type Frequencies = Vec<i64>;

pub fn frequencies_from_string(raw: &str) -> Frequencies {
    raw
        .trim()
        .split("\n")
        .map(|line| line.parse::<i64>().unwrap())
        .collect()
}

pub fn final_frequency(frequencies: &Frequencies) -> i64 {
    frequencies.into_iter().sum()
}

pub fn first_repeated_frequency(frequencies: &Frequencies) -> i64 {
    let mut frequency: i64 = 0;
    let mut seen_frequencies: HashSet<i64> = HashSet::new();
    seen_frequencies.insert(frequency);

    loop {
        for f in frequencies {
            frequency = frequency + f;
            if seen_frequencies.contains(&frequency) {
                return frequency;
            } else {
                seen_frequencies.insert(frequency);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frequencies_from_string() {
        assert_eq!(frequencies_from_string(&"-1\n-2"), vec![-1, -2]);
        assert_eq!(frequencies_from_string(&"-1\n-2\n"), vec![-1, -2]);
        assert_eq!(frequencies_from_string(&"+1\n-2\n+3\n+1"), vec![1, -2, 3, 1]);
        assert_eq!(frequencies_from_string(&"+1\n-2\n+3\n+1\n"), vec![1, -2, 3, 1]);
    }

    #[test]
    fn test_final_frequency() {
        assert_eq!(final_frequency(&vec![1, -2, 3, 1]), 3);
        assert_eq!(final_frequency(&vec![1, 1, 1]), 3);
        assert_eq!(final_frequency(&vec![1, 1, -2]), 0);
        assert_eq!(final_frequency(&vec![-1, -2, -3]), -6);
    }

    #[test]
    fn test_first_repeated_frequency() {
        assert_eq!(first_repeated_frequency(&vec![1, -2, 3, 1, 1, -2, 3, 1]), 2);
        assert_eq!(first_repeated_frequency(&vec![1, -1]), 0);
        assert_eq!(first_repeated_frequency(&vec![3, 3, 4, -2, -4]), 10);
        assert_eq!(first_repeated_frequency(&vec![-6, 3, 8, 5, -6]), 5);
        assert_eq!(first_repeated_frequency(&vec![7, 7, -2, -7, -4]), 14);
    }
}
