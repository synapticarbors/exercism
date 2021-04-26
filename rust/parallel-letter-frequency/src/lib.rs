use std::cmp;
use std::collections::HashMap;
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    if input.len() == 1 {
        return process_lines(input[0]);
    }

    let chunksz = cmp::max(1, input.len() / worker_count);
    let mut threads = vec![];

    for batch in input.chunks(chunksz) {
        let work = batch.join("");
        threads.push(thread::spawn(move || process_lines(&work)));
    }

    threads.into_iter().fold(HashMap::new(), |mut acc, t| {
        if let Ok(x) = t.join() {
            for (k, v) in x {
                *acc.entry(k).or_insert(0) += v;
            }
        }
        acc
    })
}

fn process_lines(input: &str) -> HashMap<char, usize> {
    input
        .chars()
        .filter(|c| c.is_alphabetic())
        .fold(HashMap::new(), |mut acc, c| {
            if let Some(lc) = c.to_lowercase().next() {
                *acc.entry(lc).or_insert(0) += 1;
            }
            acc
        })
}
