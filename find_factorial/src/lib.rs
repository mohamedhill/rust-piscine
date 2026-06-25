pub fn factorial(num: u64) -> u64 {
let mut res:u64 = 1 ;

if num <= 1{
   return  1
}
for i in 1..=num{
    res*=i
};
res

}