pub fn str_len(s:&str ) -> usize {
let mut count:usize = 0;
for i in s.chars() {

    count+=1;
}
count
}