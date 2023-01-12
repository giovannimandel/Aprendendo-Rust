/* 
format! write formatted text to String
print! same as format! but the text is printed to the console (io::strout)
println! same as print! but newline is appended
eprint! same as print! but the text is printed to the standard error (io::stderr)
eprintln! same as eprint! but newline is appended
*/

fn main(){
    // println!("{} dias", 31);


    // println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    
    // println!("{subject} {verb} {object}",
    //          object = "the lazy dog",
    //          subject = "the quick brown fox",
    //          verb =  "jumps over");

    // println!("Base 10:                 {}", 69420);
    // println!("Base 2 (binary):         {:b}", 69420);
    // println!("Base 8 (octal):          {:o}", 69420);
    // println!("Base 16 (hexadecimal):   {:x}", 69420);
    // println!("Base 16 (hexadecimal):   {:X}", 69420);

    println!("{number:>5")
}