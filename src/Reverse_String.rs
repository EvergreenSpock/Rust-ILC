fn main(){
    let s = "hello world";
    let reversed = s.chars().rev().collect::<String>();
    return println!("{}", reversed);
}