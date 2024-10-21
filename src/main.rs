use std::io::Write;
mod food_data;
use self::food_data as FD;
use std::fs;

fn main() {
    // Define input variables
   let mut protein = String::new();
   let mut carbs = String::new();
   let mut fats = String::new();

    // Get user macros  
    println!("Enter in your desired macro nutrient values!"); 
    print!("Protein (g): ");
    std::io::stdout().flush().unwrap();
    let _ = std::io::stdin().read_line(&mut protein).expect("Failed to read line");
    
    print!("Carbs (g): ");
    std::io::stdout().flush().unwrap();
    let _ = std::io::stdin().read_line(&mut carbs).expect("Failed to read line");
    
    print!("Fats (g): "); 
    std::io::stdout().flush().unwrap();
    let _ = std::io::stdin().read_line(&mut fats).expect("Failed to read line");

    // Parse user macros, overwrites string variables with floats 
    let protein: f32 = protein.trim().parse().expect("Invalid protein value");
    let carbs: f32 = carbs.trim().parse().expect("Invalid carbs value");
    let fats: f32 = fats.trim().parse().expect("Invalid fats value");

    // Create user macros struct
    let user_macros = FD::UserMacros::new(protein, carbs, fats);


    // If fat to protein ratio is greater than 2.5 to 23
    // those are the macros for the fattiest low fat protein option
    // so 2.48/23.4 = 0.10598
    let ratio: f32 = user_macros.get_fats() / user_macros.get_protein();
    let data_path = "data/food_items.json";
    let json_data = fs::read_to_string(data_path).expect("Failed to read file");

    // Parse the json file data into vectors based on category of food
    //Function fiters protein options based on fat to protein ratio

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


    println!("Proteins:");
    for _protein in proteins_vec {
        println!("{:?}", _protein);
    }

    println!("Carbs:");
    for _carb in carbs_vec {
        println!("{:?}", _carb);
    }

    println!("Fats:");
    for _fat in fats_vec {
        println!("{:?}", _fat);
    }

    // Print out the vectors content 

    // Manipulate the food amunts so the macros scale to the users input
}
