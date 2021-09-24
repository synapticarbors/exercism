pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split(|c: char| c.is_ascii_whitespace() || c == '_' || c == '-')
        .flat_map(|w| {
            // take the first character in the word and any other uppercase
            // characters that are preceded by an intervening lowercase
            w.chars().take(1).chain(
                w.chars()
                    .skip_while(char::is_ascii_uppercase)
                    .filter(char::is_ascii_uppercase),
            )
        })
        .collect::<String>()
        .to_ascii_uppercase()
}
