pub fn check_ms(message: &str) -> Result<&str, &str> {
match message{
""=>Err("ERROR: illegal"),
"stupid"=>Err("ERROR: illegal"),
_=>Ok(message),

  
}


  
}