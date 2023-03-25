// make a constant called STARTING_MISSILES and READY_AMOUNT
// make them type i32 and initialize missiles and ready variables from them
const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;

fn main() {
    
    // initialize missiles and ready variables in one line and annotate them as i32
    let (missiles, ready): (i32, i32) = (STARTING_MISSILES, READY_AMOUNT);

    // print out the number of missiles and ready missiles
    println!("Firing {} of my {} missiles...", ready, missiles);
    println!("{} missiles left", missiles - ready);
}