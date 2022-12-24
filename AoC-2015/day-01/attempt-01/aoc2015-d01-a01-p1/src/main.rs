use std::fs;

fn main() {
    // Read in the contents of the file.
    let file_contents = fs::read_to_string("./../../data-1-and-2/data-1-and-2.txt").unwrap();

    // Convert the file from a String to a Chars iterator.
    let parentheses = file_contents.chars();

    // A tracking variable to count the current floor that Santa is on.
    let mut count: i32 = 0;
    
    // Loop through all characters within the file input.
    for symbol in parentheses {
        
        // Match the character and either increment or decrement the count. If the character is not expected, then print out as such.
        match symbol{
            '(' => count += 1,
            ')' => count -= 1,
            _ => println!("The input, {}, was not expected.", symbol),
        };

    }

    // Output the final floor from the tracker variable.
    println!("The final floor is {}.", count);
}
