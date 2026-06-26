pub fn first_subword(mut s: String) -> String {

    let mut  ff = String::new();

    for (v,i) in s.chars().enumerate(){

     

        if (i.is_uppercase() || i == '_' || i == ' ')&& (v !=0){

            return ff;
    }else{
        ff.push(i);
    }
}
return ff ;
}