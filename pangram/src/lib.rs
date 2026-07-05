pub fn is_pangram(s: &str) -> bool {
   let alp = String::from("abcdefjhigklmnopqrstuvwxyz");
    for i in alp.chars(){

        if !s.contains(i){
            return false
        }

    }
true

}