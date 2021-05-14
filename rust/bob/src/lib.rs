pub fn reply(message: &str) -> &str {
    let all_caps = message
        .chars()
        .filter(char::is_ascii_alphabetic)
        .all(char::is_uppercase)
        && message.as_bytes().iter().any(u8::is_ascii_alphabetic);

    let m = message.trim();

    if m.is_empty() {
        "Fine. Be that way!"
    } else if m.ends_with('?') {
        if all_caps {
            "Calm down, I know what I'm doing!"
        } else {
            "Sure."
        }
    } else if all_caps {
        "Whoa, chill out!"
    } else {
        "Whatever."
    }
}
