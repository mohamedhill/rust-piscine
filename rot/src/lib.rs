    pub fn rotate(input: &str, key: i8) -> String {
    let rot = key.rem_euclid(26) as u8;



    let mut res = String::new();

    for i in input.chars(){

    if i.is_ascii_lowercase(){

        let char = ((((i as u8  - b'a')+rot)%26)+b'a') as char;
        res.push(char)
    }else if i.is_ascii_uppercase(){

        let char = ((((i as u8  - b'A')+rot)%26)+b'A') as char;
        res.push(char)
    }else{

        res.push(i)
    }

    }
    res


    }