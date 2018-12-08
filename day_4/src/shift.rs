use regex::Regex;

#[derive(Debug)]
pub struct Shift {
    pub date: String,
    pub guard_id: usize,
    pub statuses: Statuses,
}

pub type Statuses = Vec<Status>;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Status {
    Awake,
    Asleep,
}

impl Shift {
    pub fn parse_all(strings: &Vec<String>) -> Vec<Shift> {
        let new_shift_regex = Regex::new(r"\[(?P<date>.{10}) (?P<hour>\d{2}):(?P<minute>\d{2})\] Guard #(?P<guard_id>\d+) begins shift").unwrap();
        let asleep_regex = Regex::new(r"\[(?P<date>.{10}) (?P<hour>\d{2}):(?P<minute>\d{2})\] falls asleep").unwrap();
        let awake_regex = Regex::new(r"\[(?P<date>.{10}) (?P<hour>\d{2}):(?P<minute>\d{2})\] wakes up").unwrap();

        let mut shifts = vec![];
        let mut i = 0;

        // While there's data left
        while i < strings.len() {

            // Parse this shift's info
            let mut statuses = vec![Status::Awake; 60];
            let new_shift = new_shift_regex.captures(&strings.get(i).unwrap()).unwrap();

            // Temporary Vector to hold the awake/asleep events for this shift
            let mut shift_data = vec![];

            i += 1;
            while i < strings.len() {
                // Get next string
                let string = strings.get(i).unwrap();

                if let Some(_) = new_shift_regex.captures(&string) {
                    break;
                } else if let Some(asleep) = asleep_regex.captures(&string) {
                    shift_data.push(asleep);
                } else if let Some(awake) = awake_regex.captures(&string) {
                    shift_data.push(awake);
                }

                i += 1;
            }

            let mut j = 0;
            while j < shift_data.len() {
                let nap_start: usize = shift_data.get(j).unwrap()["minute"].parse().unwrap();
                let nap_end: usize = shift_data.get(j + 1).unwrap()["minute"].parse().unwrap();

                for nap_minute in nap_start..nap_end {
                    statuses[nap_minute] = Status::Asleep;
                }

                j += 2;
            }

            let shift = Shift {
                date: new_shift["date"].to_owned(),
                guard_id: new_shift["guard_id"].parse().unwrap(),
                statuses,
            };

            shifts.push(shift);
        }

        shifts
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_all() {
        let lines = vec![
            "[1518-11-01 00:00] Guard #10 begins shift".to_string(),
            "[1518-11-01 00:05] falls asleep".to_string(),
            "[1518-11-01 00:25] wakes up".to_string(),
            "[1518-11-01 00:30] falls asleep".to_string(),
            "[1518-11-01 00:55] wakes up".to_string(),
            "[1518-11-01 23:58] Guard #99 begins shift".to_string(),
            "[1518-11-02 00:40] falls asleep".to_string(),
            "[1518-11-02 00:50] wakes up".to_string(),
            "[1518-11-03 00:05] Guard #10 begins shift".to_string(),
            "[1518-11-03 00:24] falls asleep".to_string(),
            "[1518-11-03 00:29] wakes up".to_string(),
            "[1518-11-04 00:02] Guard #99 begins shift".to_string(),
            "[1518-11-04 00:36] falls asleep".to_string(),
            "[1518-11-04 00:46] wakes up".to_string(),
            "[1518-11-05 00:03] Guard #99 begins shift".to_string(),
            "[1518-11-05 00:45] falls asleep".to_string(),
            "[1518-11-05 00:55] wakes up".to_string(),
        ];
        dbg!(Shift::parse_all(&lines));
    }
}
