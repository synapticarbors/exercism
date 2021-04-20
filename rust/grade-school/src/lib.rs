use std::collections::HashMap;

#[derive(Default)]
pub struct School<'a> {
    roster: HashMap<u32, Vec<&'a str>>,
}

impl<'a> School<'a> {
    pub fn new() -> School<'a> {
        School {
            ..Default::default()
        }
    }

    pub fn add(&mut self, grade: u32, student: &'a str) {
        let grade_roster = self.roster.entry(grade).or_insert_with(Vec::new);
        (*grade_roster).push(student)
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut x: Vec<u32> = self.roster.keys().copied().collect();
        x.sort_unstable();

        x
    }

    pub fn grade(&self, grade: u32) -> Vec<String> {
        match self.roster.get(&grade) {
            Some(g) => {
                let mut tmp: Vec<String> = g.iter().map(|x| x.to_string()).collect();
                tmp.sort_unstable();
                tmp
            }
            None => Vec::new(),
        }
    }
}
