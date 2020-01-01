use regex::Regex;

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub struct Point {
    pub x: isize,
    pub y: isize,
    pub z: isize,
}

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub struct Nanobot {
    pub x: isize,
    pub y: isize,
    pub z: isize,
    pub r: isize,
}

impl Nanobot {
    pub fn parse(input: &String) -> Nanobot {
        let re = Regex::new(r"pos=<(-?\d+),(-?\d+),(-?\d+)>, r=(\d+)").unwrap();
        let caps = re.captures(input).unwrap();

        Nanobot {
            x: caps[1].parse().unwrap(),
            y: caps[2].parse().unwrap(),
            z: caps[3].parse().unwrap(),
            r: caps[4].parse().unwrap(),
        }
    }

    pub fn in_range(&self, nanobot: &Nanobot) -> bool {
        let distance = (
            (nanobot.x - self.x).abs() +
            (nanobot.y - self.y).abs() +
            (nanobot.z - self.z).abs()
        );
        distance <= self.r
    }

    pub fn points_in_range(&self) {
        for x in 0..=self.r {
            for y in 0..=self.r {
                for z in 0..=self.r {

                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nanobot_parse() {
        assert_eq!(Nanobot::parse(&"pos=<2,-1,4>, r=3".to_string()), Nanobot {
            x: 2,
            y: -1,
            z: 4,
            r: 3,
        });
    }
}
