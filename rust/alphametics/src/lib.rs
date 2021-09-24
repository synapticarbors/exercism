use std::collections::{HashMap, HashSet};

use itertools::Itertools;

fn get_unique_letters(input: &str) -> Vec<char> {
    input
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .collect::<HashSet<char>>()
        .into_iter()
        .collect()
}

fn word2num(w: &str, hm: &HashMap<char, u8>) -> u32 {
    w.chars().rev().enumerate().fold(0, |mut acc, (i, c)| {
        acc += (*hm.get(&c).unwrap() as u32) * 10u32.pow(i as u32);
        acc
    })
}

fn check_solution(addends: &[String], soln: &str, hm: &HashMap<char, u8>) -> bool {
    let num_soln = word2num(soln, hm);

    let sum_addends = addends.iter().fold(0, |mut acc, word| {
        acc += word2num(word, hm);
        acc
    });

    num_soln == sum_addends
}

fn get_leading_letters(addends: &[String], soln: &str) -> HashSet<char> {
    let mut x = HashSet::new();
    x.insert(soln.chars().next().unwrap());
    for word in addends {
        x.insert(word.chars().next().unwrap());
    }

    x
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let letters = get_unique_letters(input);
    let nletters = letters.len();

    let (addends, soln) = input.split_once("==")?;
    let addends: Vec<String> = addends.split('+').map(|a| a.trim().to_string()).collect();
    let soln = soln.trim();

    let mut hm: HashMap<char, u8> = letters.iter().fold(HashMap::new(), |mut acc, x| {
        acc.insert(*x, 0u8);
        acc
    });

    let leading_letters = get_leading_letters(&addends, &soln);

    for ix in (0..10).permutations(nletters) {
        for (i, key) in letters.iter().enumerate() {
            if let Some(x) = hm.get_mut(key) {
                *x = ix[i];
            } else {
                unreachable!();
            }
        }

        // Check for leading zeros
        if hm
            .iter()
            .any(|(k, v)| *v == 0 && leading_letters.contains(k))
        {
            continue;
        }

        if check_solution(&addends, &soln, &hm) {
            return Some(hm);
        }
    }

    None
}
