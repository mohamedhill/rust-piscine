pub fn capitalize_first(input: &str) -> String {
    let mut res=String::new();
    for (i,c) in input.chars().enumerate(){
        if i==0{
            let t :String=c.to_uppercase().collect();
            res.push_str(&t);
        }else{
          res.push(c);
        }
    } 
    res
}

pub fn title_case(input: &str) -> String {
    let mut res = String::with_capacity(input.len());
    let mut new_word = true;
    for ch in input.chars() {
        if ch.is_whitespace() {
            new_word = true;
            res.push(ch);
            continue;
        }
        let out = if new_word {
            new_word = false;
            ch.to_ascii_uppercase()
        } else {
            ch.to_ascii_lowercase()
        };
        res.push(out);
    }
    res
}

pub fn change_case(input: &str) -> String {
    input.chars()
    .map(|c| {
        if c.is_uppercase(){
            c.to_ascii_lowercase()
        }else if c.is_lowercase(){
            c.to_ascii_uppercase()
        }else{
            c
        }
    }).collect()
}