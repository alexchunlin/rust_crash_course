// Silence some warnings that could distract from the exercise
#![allow(unused_variables, unused_mut, dead_code)]

// Someone is shooting arrows at a target.  We need to classify the shots.
//
// 1a. Create an enum called `Shot` with variants:
// - `Bullseye`
// - `Hit`, containing the distance from the center (an f64)
// - `Miss`
//
// You will need to complete 1b as well before you will be able to run this program successfully.

// create a shot enum 
enum Shot {
    Bullseye,
    Hit(f64),
    Miss,
}

// implement the points method for the shot enum
impl Shot {
    // convert a shot into points
    fn points(self) -> i32 {
        // return 5 points if `self` is a `Shot::Bullseye`
        // return 2 points if `self` is a `Shot::Hit(x)` where x < 3.0
        // return 1 point if `self` is a `Shot::Hit(x)` where x >= 3.0
        // return 0 points if `self` is a Miss
        match self {
            Shot::Bullseye => 5,
            Shot::Hit(x) if x < 3.0 => 2,
            Shot::Hit(x) if x >= 3.0 => 1,
            Shot::Hit(_) => unreachable!("Hit should be handled by the above two cases"),
            Shot::Miss => 0,
        }
    }
}

fn main() {
    // Simulate shooting a bunch of arrows and gathering their coordinates on the target.
    let arrow_coords: Vec<Coord> = get_arrow_coords(5);
    let mut shots: Vec<Shot> = Vec::new();

    // iterate over arrow_coords
    for coord in arrow_coords {
        // call `coord.print_description()`
        coord.print_description();
        // append the correct variant of `Shot` to the `shots` vector depending on the value of
        // `coord.distance_from_center()`:
        // Less than 1.0 -- `Shot::Bullseye`
        // Between 1.0 and 5.0 -- `Shot::Hit(value)`
        // Greater than 5.0 -- `Shot::Miss`
        let this_shot = if coord.distance_from_center() < 1.0 {
            Shot::Bullseye
        } else if coord.distance_from_center() < 5.0 {
            Shot::Hit(coord.distance_from_center())
        } else {
            Shot::Miss
        };
        shots.push(this_shot);
    }
    let mut total = 0;
    for shot in shots {
        // call `shot.points()` and add the result to `total`
        total += shot.points();
    }
    println!("Final point total is: {}", total);
}

// A coordinate of where an Arrow hit
#[derive(Debug)]
struct Coord {
    x: f64,
    y: f64,
}

impl Coord {
    fn distance_from_center(&self) -> f64 {
        (self.x.powf(2.0) + self.y.powf(2.0)).sqrt()
    }
    fn print_description(&self) {
        println!(
            "coord is {:.1} away, at ({:.1}, {:.1})",
            self.distance_from_center(),
            self.x,
            self.y);
    }

}

// Generate some random coordinates
fn get_arrow_coords(num: u32) -> Vec<Coord> {
    let mut coords: Vec<Coord> = Vec::new();
    for _ in 0..num {
        let coord = Coord {
            x: (rand::random::<f64>() - 0.5) * 12.0,
            y: (rand::random::<f64>() - 0.5) * 12.0,
        };
        coords.push(coord);
    }
    coords
}