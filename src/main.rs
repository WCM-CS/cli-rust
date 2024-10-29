//use std::io::Write;
mod food_data;
use self::food_data as FD; 
use std::fs;
use rand::Rng;

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


    println!("Proteins:");
    for _protein in &proteins_vec {
        println!("{:?}", _protein);
    }

    println!("Carbs:");
    for _carb in &carbs_vec {
        println!("{:?}", _carb);
    }

    println!("Fats:");
    for _fat in &fats_vec {
        println!("{:?}", _fat);
    }

    // Print out the vectors content 

    // Manipulate the food amunts so the macros scale to the users input
    let mut rng = rand::thread_rng();
    // Set struct for the output macros
    let mut output_macros = FD::UserMacros::new(0.0, 0.0, 0.0);

    while true {
        // Define meals macros
        let mut proteins = 0.0; 
        let mut carbs = 0.0; 
        let mut fats = 0.0;

        // Grab the random idnex for the foods
        let protein_index = rng.gen_range(0..proteins_vec.len() - 1);
        let carb_index = rng.gen_range(0..carbs_vec.len() - 1);
        let fats_index = rng.gen_range(0..fats_vec.len() - 1);

        // Select and set protein foods values
        let protein_food = &proteins_vec[protein_index];
        let scaling_protein_factor = user_macros.get_protein() / protein_food.get_protein();

        proteins += protein_food.get_protein() * scaling_protein_factor;
        carbs += protein_food.get_carbs() * scaling_protein_factor;
        fats += protein_food.get_fats() * scaling_protein_factor;

        // Select and set carbs foods values
        let carb_food = &carbs_vec[carb_index];
        let scaling_carb_factor = user_macros.get_carbs() / carb_food.get_carbs();

        proteins += carb_food.get_protein() * scaling_carb_factor;
        carbs += carb_food.get_carbs() * scaling_carb_factor;
        fats += carb_food.get_fats() * scaling_carb_factor;

        // Check if protein surpasses the limit, if so reduce protein
        let protein_max = user_macros.get_protein() * 1.05;
        if proteins > protein_max {
            // rescale proteins with lower value
            let excess_protein = proteins - protein_max;
            let rescale_factor = excess_protein / protein_food.get_protein();
            proteins -= protein_food.get_protein() * rescale_factor;
            carbs -= protein_food.get_carbs() * rescale_factor;
            fats -= protein_food.get_fats() * rescale_factor;
        }

        let fat_max = user_macros.get_fats() * 1.05;
        let fat_min = user_macros.get_fats() * 0.95;
        let fat_food = &fats_vec[fats_index];

        // If fats are too high, recall the loop
        if fats > fat_max {
            continue;
        }

        // If fats are too low, add fats
        if fats < fat_min {
            let excess_fat = user_macros.get_fats() - fats;
            let rescale_factor = excess_fat / fat_food.get_fats();
            fats += fat_food.get_fats() * rescale_factor;

        }

        // If fats are in range, procees
        if fats <= fat_max && fats >= fat_min {
            output_macros.set_protein(proteins);
            output_macros.set_carbs(carbs);
            output_macros.set_fats(fats);
            break;
        }
    }

    println!("Proteins: {}", output_macros.get_protein());
    println!("Carbs: {}", output_macros.get_carbs());
    println!("Fats: {}", output_macros.get_fats());
}
