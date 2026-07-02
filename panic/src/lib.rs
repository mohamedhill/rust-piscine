use std::fs::File;

pub fn open_file(s: &str) -> File {
    let file = File::open(s);

    match file{
       Ok(_) => return file.expect("REASON"),
            Err(err)=>panic!()
    }
}