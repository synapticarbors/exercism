pub fn raindrops(n: u32) -> String {
    let mut s = "".to_string();

    if n % 3 == 0 {
        s.push_str("Pling");
    }

    if n % 5 == 0 {
        s.push_str("Plang");
    }

    if n % 7 == 0 {
        s.push_str("Plong");
    }

    if s.is_empty() {
        n.to_string()
    } else {
        s
    }
}