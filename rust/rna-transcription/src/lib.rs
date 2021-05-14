#[derive(Debug, PartialEq)]
pub struct Dna(String);

#[derive(Debug, PartialEq)]
pub struct Rna(String);

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        for (i, c) in dna.chars().enumerate() {
            match c {
                'A' | 'C' | 'T' | 'G' => (),
                _ => return Err(i),
            }
        }
        Ok(Dna(dna.to_string()))
    }

    pub fn into_rna(self) -> Rna {
        let s = self
            .0
            .chars()
            .map(|c| match c {
                'A' => 'U',
                'T' => 'A',
                'C' => 'G',
                'G' => 'C',
                _ => unreachable!(),
            })
            .collect::<String>();

        Rna(s)
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        for (i, c) in rna.chars().enumerate() {
            match c {
                'A' | 'C' | 'U' | 'G' => (),
                _ => return Err(i),
            }
        }
        Ok(Rna(rna.to_string()))
    }
}
