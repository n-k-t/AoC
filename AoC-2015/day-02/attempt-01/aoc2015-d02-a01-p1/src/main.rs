use std::fs;

fn main() {
    // Read in the data file as a String.
    let file_contents = fs::read_to_string("./../../data-1-and-2/data-1-and-2.txt").unwrap();

    // Convert the data into a Lines iterator.
    let single_lines = file_contents.lines();

    // A variable to keep track of the total area.
    let mut area_tracker: u32 = 0;

    // Loop through the data and collect each present into a sorted vector.
    for line in single_lines {
        let dimension = line.split('x');
        let mut line_dims: Vec<u32> = dimension.map(|dim| dim.parse::<u32>().unwrap()).collect();
        line_dims.sort();

        // Calculate the total area with an excess area of the smallest side.
        area_tracker += (3 * (line_dims[0] * line_dims[1])) + (2 * (line_dims[0] * line_dims[2])) + (2 * (line_dims[1] * line_dims[2]));
    }

    // Output the total area of wrapping paper that the elves require.
    println!("The total amount of wrapping paper (in square feet) that they should order is {}.", area_tracker);
}
