pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut res = Vec::new();

    for name in names {
        let mut curr = String::new();

        for part in name.split_whitespace() {
            curr.push(part.chars().next().unwrap());
            curr.push('.');
            curr.push(' ');
        }

        res.push(curr.trim_end().to_string());
    }

    res
}