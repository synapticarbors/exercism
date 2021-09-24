use std::collections::{HashMap, HashSet};

const BASE_PRICE: u32 = 800;

fn basket_cost(contents: &[u32]) -> u32 {
    let nbooks = contents.len() as u32;
    let nunique = contents.iter().copied().collect::<HashSet<u32>>().len() as u32;

    let discount = match nunique {
        1 => 0.0,
        2 => 0.05,
        3 => 0.1,
        4 => 0.2,
        5 => 0.25,
        _ => unreachable!(),
    };

    let discount_price = (BASE_PRICE as f64 * (1.0 - discount)) as u32;

    discount_price * nunique + BASE_PRICE * (nbooks - nunique)
}

fn build_baskets(nbaskets: u32, book_counts: &[(u32, u32)], max_books: u32) -> Vec<Vec<u32>> {
    let mut baskets = vec![];
    for _ in 0..nbaskets {
        baskets.push(vec![]);
    }

    for (book, cnt) in book_counts {
        for i in 0..(*cnt as usize) {
            let mut placed = false;
            for b in baskets.iter_mut().take(nbaskets as usize).skip(i) {
                if b.len() < (max_books as usize) {
                    b.push(*book);
                    placed = true;
                    break;
                }
            }

            if !placed {
                baskets[(nbaskets as usize) - 1].push(*book);
            }
        }
    }

    baskets
}

pub fn lowest_price(books: &[u32]) -> u32 {
    if books.is_empty() {
        return 0;
    }

    let book_counts: HashMap<u32, u32> = books.iter().fold(HashMap::new(), |mut acc, x| {
        *acc.entry(*x).or_default() += 1;
        acc
    });

    let mut book_counts_vec: Vec<(u32, u32)> = book_counts.into_iter().collect();
    book_counts_vec.sort_by(|a, b| b.1.cmp(&a.1));

    let nparitions = book_counts_vec[0].1;

    let mut min_cost = u32::MAX;
    for max_books in 1..=5 {
        let baskets = build_baskets(nparitions, &book_counts_vec, max_books);

        let cost = baskets.iter().fold(0, |mut acc, b| {
            acc += basket_cost(&b);
            acc
        });

        if cost < min_cost {
            min_cost = cost;
        }
    }

    min_cost
}
