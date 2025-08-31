use regex::Regex;
use std::env;
use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if the input file is provided
    if args.len() != 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        std::process::exit(1);
    }

    let input_path = &args[1];

    // Open the input file
    let mut file = match File::open(input_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error opening file '{}': {}", input_path, e);
            std::process::exit(1);
        }
    };

    // Read the entire contents of the file into a string
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Define the regex pattern to match:
    // - mul(X,Y): captures X and Y as separate groups
    // - do(): no captures
    // - don't(): no captures
    let mul_pattern = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();

    // Initialize the total sum
    let mut total_sum: i32 = 0;

    // Initialize the state: mul instructions are enabled initially
    let mut mul_enabled = true;

    // Iterate over all matches of the regex in the input contents
    for cap in mul_pattern.captures_iter(&contents) {
        if let Some(mul_match) = cap.get(0) {
            let instruction = mul_match.as_str();

            // Check which instruction was matched
            if instruction.starts_with("mul(") {
                if mul_enabled {
                    // Extract X and Y from the capture groups
                    let x_str = &cap[1];
                    let y_str = &cap[2];

                    // Parse the captured strings into integers
                    let x: i32 = match x_str.parse() {
                        Ok(num) => num,
                        Err(_) => continue, // Skip if parsing fails
                    };
                    let y: i32 = match y_str.parse() {
                        Ok(num) => num,
                        Err(_) => continue, // Skip if parsing fails
                    };

                    // Perform the multiplication
                    let product = x * y;

                    // Add the product to the total sum
                    total_sum += product;

                    // (Optional) Print each valid multiplication and its product
                    // Uncomment the line below for detailed output
                    // println!("mul({}, {}) = {}", x, y, product);
                }
            } else if instruction == "do()" {
                // Enable mul instructions
                mul_enabled = true;
            } else if instruction == "don't()" {
                // Disable mul instructions
                mul_enabled = false;
            }
            // Any other instruction is already ignored by the regex
        }
    }

    // Output the final total sum
    println!("Total sum of multiplications: {}", total_sum);

    Ok(())
}
