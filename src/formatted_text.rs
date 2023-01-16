fn main() {
        //{} allows for formatting

    println!("{} days", 32);

    //Alice and Bob after the first string statement are counted as array variables which can then be put back into the statement.
    println!("{0}, this is {1}. {1}, this is {0}","Alice", "Bob");

    //Named arguments are 
    println!("{subject} {verb} {object}",
    object="the lazy dog",
    subject="the quick brown fox",
    verb="jumps over");

    println!("Base 10:                  {0}", 29456);
    println!("Base 2 (binary):          {:b}", 29456);
    println!("Base 8 (octal):           {:o}", 29456);
    println!("Base 16 (hexidecimal):    {:x}", 29456);


}