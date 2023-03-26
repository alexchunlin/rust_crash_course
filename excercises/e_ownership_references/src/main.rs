// Silence some warnings so they don't distract from the exercise.
#![allow(unused_mut, unused_variables)]

fn main() {
    // This fancy stuff either gets the first argument as a String, or prints
    // usage and exits if an argument was not supplied to the program.
    let mut arg: String = std::env::args().nth(1).unwrap_or_else(|| {
        println!("Please supply an argument to this program.");
        std::process::exit(-1);
    });

    // checks if an input arg ends with an s and prints a message accordingly
    fn inspect(arg: &String){
        if arg.ends_with("s"){
            println!("{} is plural", arg);
        } else {
            println!("{} is singular", arg);
        }
    }
    inspect(&arg);

    // adds an s to the end of a string if it doesn't already end with an s
    fn change(arg: &mut String){
        if !arg.ends_with("s"){
            arg.push_str("s");
        }
    }
    change(&mut arg);
    println!("I have many {}", arg);

    // checks if an input arg starts with a b and contains an a
    fn eat(arg: String) -> bool{
        if arg.starts_with("b") && arg.contains("a"){
            true
        } else {
            false
        }
    }

    if eat(arg) {
       println!("Might be bananas");
    } else {
       println!("Not bananas");
    }

    // accepts a mutable string reference and changes the actual string to "sparkly"
    fn bedazzle(arg: &mut String){
        *arg = "sparkly".to_string();
    }

    let mut material = "mud".to_string();
    println!("This material is just `{}`.", material);
    bedazzle(&mut material);
    println!("Wow! Now the material is `{}`!", material);
}
