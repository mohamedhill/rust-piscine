use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

pub fn open_or_create<P: AsRef<Path>>(path: &P, content: &str) {
    let file = File::open(path);

    match file{
       Ok(mut file) => file.write_all(content.as_bytes())?;,
            Err(_)=>{
                
           let mut file = File::create(path)?;
           file.write_all(content.as_bytes())?;
        }
        Ok(())
    }



}