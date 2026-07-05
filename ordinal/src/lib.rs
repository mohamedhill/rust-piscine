pub fn num_to_ordinal(n: u32) -> String {
    let suffix;

    if n % 100 == 11 || n % 100 == 12 || n % 100 == 13 {
        suffix = "th";
    } else if n % 10 == 1 {
        suffix = "st";
    } else if n % 10 == 2 {
        suffix = "nd";
    } else if n % 10 == 3 {
        suffix = "rd";
    } else {
        suffix = "th";
    }

    format!("{}{}", n, suffix)
}