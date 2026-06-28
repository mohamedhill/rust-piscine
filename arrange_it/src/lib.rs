pub fn arrange_phrase(phrase: &str) -> String {
    let mut words = phrase
        .split_whitespace()
        .map(|word| {
            let mut number = 0;
            let text: String = word
                .chars()
                .filter(|c| {
                    if c.is_ascii_digit() {
                        number = c.to_digit(10).unwrap();
                        false
                    } else {
                        true
                    }
                })
                .collect();

            (number, text)
        })
        .collect::<Vec<_>>();

    words.sort_by(|a, b| a.0.cmp(&b.0));

    words
        .into_iter()
        .map(|(_, word)| word)
        .collect::<Vec<_>>()
        .join(" ")
}