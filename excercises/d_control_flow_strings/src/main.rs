// Silence some warnings so they don't distract from the exercise.
#![allow(dead_code, unused_mut, unused_variables)]

fn main() {
    // This collects any command-line arguments into a vector of Strings.
    // For example:
    //
    //     cargo run apple banana
    //
    // ...produces the equivalent of
    //
    //     vec!["apple".to_string(), "banana".to_string()]
    let args: Vec<String> = std::env::args().skip(1).collect();

    // This consumes the `args` vector to iterate through each String
    for arg in args {
        // if arg is "sum", call sum function
        if arg == "sum"{
            sum();
        }
        // if arg is "double", call double function
        else if arg == "double"{
            double();
        }
        // if arg is anything else, call count function
        else if arg == "bananas"{
            bananas(arg);
        }
    }
}

fn sum() {
    let mut sum = 0;

    // sums values 7 to 23 inclusive to the sum variable
    for i in 7..=23{
        sum += i;
    }

    println!("The sum is {}", sum);
}

fn double() {
    let mut count = 0;
    let mut x = 1;
    // make a while loop that doubles x until it is larger than 500 and counts the number of doubles.
    while x <= 500 {
        x*=2;
        count +=1;
    }
    println!("You can double X {} times until X is larger than 500", count);
}

fn bananas(arg: String) {
    // Challenge: Use an unconditional loop (`loop`) to print `arg` 8 times, and then break.
    // You will need to count your loops, somehow.  Run it with `cargo run bananas`
    //
    let mut loop_count = 0;
    // make a tagged loop named george
    'george: loop {
        if loop_count > 7 {
            break 'george;
        }
        else {
            print!("{} ", arg);
            loop_count += 1;
        }
    }
    println!(); // This will output just a newline at the end for cleanliness.
}
