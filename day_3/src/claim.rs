use regex::Regex;

#[derive(Debug, Clone)]
pub struct Claim {
    pub id: usize,
    pub left: usize,
    pub top: usize,
    pub width: usize,
    pub height: usize,
}

impl Claim {
    pub fn parse(string: &String) -> Claim {
        let re = r"#(?P<id>\d+) @ (?P<left>\d+),(?P<top>\d+): (?P<width>\d+)x(?P<height>\d+)";
        let regex = Regex::new(re).unwrap();
        let values = regex.captures(&string).unwrap();

        Claim {
            id: values["id"].parse().unwrap(),
            left: values["left"].parse().unwrap(),
            top: values["top"].parse().unwrap(),
            width: values["width"].parse().unwrap(),
            height: values["height"].parse().unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let claim_1 = Claim::parse(&"#1 @ 2,3: 4x5".to_string());
        assert_eq!(claim_1.id, 1);
        assert_eq!(claim_1.left, 2);
        assert_eq!(claim_1.top, 3);
        assert_eq!(claim_1.width, 4);
        assert_eq!(claim_1.height, 5);

        let claim_2 = Claim::parse(&"#10 @ 11,12: 13x14".to_string());
        assert_eq!(claim_2.id, 10);
        assert_eq!(claim_2.left, 11);
        assert_eq!(claim_2.top, 12);
        assert_eq!(claim_2.width, 13);
        assert_eq!(claim_2.height, 14);
    }
}
