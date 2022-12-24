use std::fs;

fn main() {
    // Read in the contents of the file.
    let file_contents = fs::read_to_string("./../../data-1-and-2/data-1-and-2.txt").unwrap();

    // Convert the file from a String to a Chars iterator.
    let parentheses = file_contents.chars();

    // A tracking variable to count the current floor that Santa is on.
    let mut count: i32 = 0;

    // A variable to track the current character and get thrown (triggered) when the count is -1.
    let mut flag: u16 = 0;
    
    // Loop through all characters within the file input.
    'looping: for symbol in parentheses {
        
        // Increment the flag.
        flag += 1;

        // Match the character and either increment or decrement the count. If the character is not expected, then print out as such.
        match symbol{
            '(' => count += 1,
            ')' => count -= 1,
            _ => println!("The input, {}, was not expected.", symbol),
        };

        // If the count drops below 0 (-1), then break out of the loop.
        if count < 0 {
            // Output the character position upon which the count drops below 0.
            println!("The character position is {}.", flag);

            // Break the loop.
            break 'looping;
        }

    }
}
