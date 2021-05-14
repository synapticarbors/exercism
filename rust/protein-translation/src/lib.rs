use std::collections::HashMap;

pub struct CodonsInfo<'a> {
    lookup: HashMap<&'a str, &'a str>,
}

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        self.lookup.get(codon).copied()
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
        rna.as_bytes()
            .chunks(3)
            .map(|x| self.name_for(std::str::from_utf8(x).unwrap()))
            .take_while(|x| *x != Some("stop codon"))
            .collect()
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    let lookup: HashMap<&'a str, &'a str> = pairs.into_iter().collect();
    CodonsInfo { lookup }
}
