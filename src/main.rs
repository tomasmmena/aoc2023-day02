use std::env;
use std::io::{self, BufRead};
use std::fs;


#[derive(Default)]
struct CubeSet {
    red: usize,
    green: usize,
    blue: usize
}

impl CubeSet {

    fn contains(&self, other: &Self) -> bool {
        self.red >= other.red && self.green >= other.green && self.blue >= other.blue
    }

    /// This function returns the min set required to produce the original set and the second 
    /// one passed as a parameter.
    /// 
    ///  # Arguments
    /// 
    /// * `other` - Other set of cubes with which to calculate the minimum set.
    fn min_set(&self, other: &Self) -> CubeSet {
        CubeSet { 
            red: self.red.max(other.red), 
            green: self.green.max(other.green), 
            blue: self.blue.max(other.blue) 
        }
    }

    /// This function returns the power of the cube set definded as the productory of all color 
    /// amounts.
    fn power(&self) -> usize {
        self.red * self.green * self.blue
    }

}

struct Game {
    id: usize,
    cube_sets: Vec<CubeSet>
}

impl Game {

    /// This function returns the minimum set of cubes required to play this game.
    fn min_set(&self) -> CubeSet {
        self.cube_sets.iter().fold(
            CubeSet::default(), 
            |first, second| first.min_set(second)
        )
    }

}

fn main() {
    let path = env::args().nth(1).expect("Required arument path missing.");

    let data: Vec<Game> = io::BufReader::new(
        fs::File::open(path).expect("Could not find the file!"))
        .lines()
        .map(|line| {
            let text = line.expect("Could not read line!");
            let (game, sets) = text.split_once(": ").expect("Bad string!");
            let (_, id) = game.split_at(game.find(" ").expect("Bad string!"));

            Game {
                id: str::parse::<usize>(id.trim()).expect("Bad id!"),
                cube_sets: sets.split(";").into_iter().map(|set_substring| {
                    let mut red: usize = 0;
                    let mut green: usize = 0;
                    let mut blue: usize = 0;

                    for color_substring in set_substring.split(", ") {
                        let (amount, color) = color_substring.trim().split_once(" ").expect("Bad string!");
                        let parsed_amount: usize = str::parse::<usize>(amount.trim()).expect("Bad amount!");
                        match color.trim() {
                            "red" => red = parsed_amount,
                            "green" => green = parsed_amount,
                            "blue" => blue = parsed_amount,
                            _ => panic!("Invalid color {}!", color)
                        }
                    }

                    CubeSet {
                        red: red,
                        green: green,
                        blue: blue
                    }
                }).collect()
            }
        }).collect();

    println!("Sum of ids: {}", data.into_iter().map(|g| g.min_set().power()).sum::<usize>())
}
