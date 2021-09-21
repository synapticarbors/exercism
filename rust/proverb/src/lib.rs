use std::iter::once;

fn build_line(a: &str, b: &str) -> String {
    format!("For want of a {} the {} was lost.\n", a, b)
}

pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        return String::new();
    }

    list.windows(2)
        .map(|w| build_line(w[0], w[1]))
        .chain(once(format!("And all for the want of a {}.", list[0])))
        .collect()
}
