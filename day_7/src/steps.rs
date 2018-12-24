use regex::Regex;

#[derive(Debug)]
pub struct StepInfo {
    pub name: char,
    pub prereq_name: char,
}

impl StepInfo {
    pub fn parse(string: String) -> StepInfo {
        let regex = Regex::new(r"Step (?P<prereq_name>.) must be finished before step (?P<name>.) can begin.").unwrap();
        let captures = regex.captures(&string).unwrap();
        StepInfo {
            name: captures["name"].chars().nth(0).unwrap(),
            prereq_name: captures["prereq_name"].chars().nth(0).unwrap(),
        }
    }
}

#[derive(Debug)]
pub struct Step {
    pub name: char,
    pub prereqs: Vec<char>,
}
