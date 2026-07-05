pub fn number_logic( num: u32) -> bool {
let power = num.to_string().len();
if power == 1{
    return true
}

let mut i = num ;
let mut  count = 1;
let mut res = 0;
while i >0{
    let number = i% 10 ;
    i/= 10;
  
    
    for _ in 0..power{
        count *=number; 

    }
    res += count ;
    count = 1 ;


}
if res == num {
    return true 
}
false

}