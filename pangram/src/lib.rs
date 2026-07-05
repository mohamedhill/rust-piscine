pub fn is_pangram(s: &str) -> bool {
    let alp = "abcdefghijklmnopqrstuvwxyz";
    let s = s.to_lowercase();

    for c in alp.chars() {
        if !s.contains(c) {
            return false;
        }
    }

    true
}
