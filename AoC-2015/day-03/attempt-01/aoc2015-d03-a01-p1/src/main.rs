use std::collections::HashMap;
use std::fs;

fn main() {
    // Read in the data file as a String.
    let file_contents = fs::read_to_string("./../../data-1-and-2/data-1-and-2.txt").unwrap();

    let directions = file_contents.chars();

    let mut position_tracker = HashMap::new();

    let mut current_position: [i32; 2] = [0, 0];

    for movement in directions {
        
        match movement {
            '^' => ,
            '>' => ,
            'v' => ,
            '<' => , 
            _ => ,
        }

    }

    // let field_name = String::from("Favorite color");
    // let field_value = String::from("Blue");

    // let mut map = HashMap::new();
    // map.insert(field_name, field_value);
    // println!("{:?}", map.get("Favorite colo"));
    // if map.get("Favorite colo") == None {
    //     println!("{}", b't');
    // }
}
