//use std::io::Write;
mod food_data;
use self::food_data as FD; 
use std::fs;

fn main() {
    // Get inputs, returns them in a struct
    let user_macros = FD::get_input_macros();

    // If fat to protein ratio is greater than 2.5 to 23
    // Those are the macros for the fattiest low fat protein option
    // 2.48/23.4 = 0.10598
    let ratio: f32 = user_macros.get_fats() / user_macros.get_protein();
    let data_path = "data/food_items.json";
    let json_data = fs::read_to_string(data_path).expect("Failed to read file");

    // Parse the json file data into vectors based on category of food
    // Function fiters protein options based on fat to protein ratio
    let (proteins_vec, carbs_vec, fats_vec) = match if ratio < 0.10598 {
        FD::load_food_data(&json_data, false) 
    } else {
        FD::load_food_data(&json_data, true)
    } {
        Ok(data) => data,
        Err(err) => {
            println!("Error Parsing json: {}", err);
            return;
        }
    };


    let mut max_attempts = 100;

    let result_macros = loop {
        // If max attempts is 0, print error and exit
        if max_attempts == 0 {
            println!("Could not find valid combination after {} attempts", max_attempts);
            return;
        }
        max_attempts -= 1;  // Decrement the counter

        // Select food options, returns a tuple of structs
        let (protein_choice, carb_choice, fat_choice) = FD::select_food_choices(&proteins_vec, &carbs_vec, &fats_vec);

        match FD::scale_food_choices(&user_macros, &protein_choice, &carb_choice, &fat_choice){
            Ok(result) => {
                println!("Valid selection found");
                break result;
            }
            Err(e) => {
                println!("Error scaling food choices: {}", e);
                continue;
            }
        }
    };
    
    // Print out the result macros
    println!("{:?}", result_macros);

}