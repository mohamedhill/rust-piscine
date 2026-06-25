use std::io;
fn main() {
    let solution= "The letter e";
    let mut count = 0;
   loop{
    count += 1;
    println!("I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?");

  let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    if input.trim() ==solution {
        println!("Number of trials: {count}");
        break;
    }

   }
}
