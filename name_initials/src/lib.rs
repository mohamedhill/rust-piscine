pub fn initials(names: Vec<&str>) -> Vec<String> {
let mut res: Vec<String> = Vec::new();

for i in names{
let parts: Vec<String> = i.split_whitespace().map(|x| x.to_string()).collect(); 
let mut curr = String::new();
for k in parts{
curr.push_str(&(k.chars().nth(0).unwrap().to_string()+&". "))


}
res.push(curr.trim().to_string())

}
res
}