#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first: &[T], second: &[T]) -> Comparison {
    match (first.len(), second.len()) {
        (0, 0) => Comparison::Equal,
        (0, _) => Comparison::Sublist,
        (_, 0) => Comparison::Superlist,
        (a, b) if a < b => {
            if second.windows(a).any(|sub| sub == first) {
                Comparison::Sublist
            } else {
                Comparison::Unequal
            }
        }
        (a, b) if a > b => {
            if first.windows(b).any(|sub| sub == second) {
                Comparison::Superlist
            } else {
                Comparison::Unequal
            }
        }
        (_, _) => {
            if first == second {
                Comparison::Equal
            } else {
                Comparison::Unequal
            }
        }
    }
}
