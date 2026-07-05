pub fn search(array: &[i32], key: i32) -> Option<usize> {
    let mut index = array.len()-1;
    for &i in array.iter().rev(){
        if i == key{
            return Some(index)

        }
        index-=1
    }
    None
}