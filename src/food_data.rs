use std::io::Write;
use serde::{Deserialize, Serialize};
use serde_json;
use rand::Rng;

//Struct to store the food data
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FoodData {
    name: String,
    category: String,
    protein: f32,
    carbs: f32,
    fats: f32  
}

impl FoodData {
    // Constructor 
    pub fn new(name: String,category: String, protein: f32, carbs: f32, fats: f32) -> Self {
        FoodData {name, category, protein, carbs, fats}
    }

    // Getters
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_category(&self) -> &str {
        &self.category
    }
    pub fn get_protein(&self) -> f32 {
        self.protein
    }
    pub fn get_carbs(&self) -> f32 {
        self.carbs
    }
    pub fn get_fats(&self) -> f32 {
        self.fats
    }
}

// Struct to store user inputs
pub struct UserMacros {
    protein: f32,
    carbs: f32, 
    fats: f32
}

// Struct to store user inputs
impl UserMacros {
    // Constructor
    pub fn new(protein: f32, carbs: f32, fats: f32) -> Self {
        UserMacros {protein, carbs, fats}
    }

    // Getters
    pub fn get_protein(&self) -> f32 {
        self.protein
    }
    pub fn get_carbs(&self) -> f32 {
        self.carbs
    }
    pub fn get_fats(&self) -> f32 {
        self.fats
    }

    // Setters
    pub fn set_protein(&mut self, protein: f32) {
        self.protein = protein;
    }
    pub fn set_carbs(&mut self, carbs: f32) {
        self.carbs = carbs;
    }
    pub fn set_fats(&mut self, fats: f32) {
        self.fats = fats;
    }
}

#[derive(Debug)]
pub struct ResultMacros {
    protein_number: f32,
    carbs_number: f32,
    fats_number: f32,
    protein_name: String,
    carb_name: String,
    fat_name: String,
    protein_grams: f32,
    carb_grams: f32,
    fat_grams: f32
}

impl ResultMacros {
    // Constructor
    pub fn new(protein_number: f32, carbs_number: f32, fats_number: f32, 
                protein_name: String, carb_name: String, fat_name: String, 
                protein_grams: f32, carb_grams: f32, fat_grams: f32) -> Self {
        ResultMacros {
            protein_number,
            carbs_number, 
            fats_number,
            protein_name,
            carb_name,
            fat_name,
            protein_grams,
            carb_grams,
            fat_grams
        }
    }

    //Getters
    pub fn get_protein(&self) -> f32 {
        self.protein_number
    }
    pub fn get_carbs(&self) -> f32 {
        self.carbs_number
    }
    pub fn get_fats(&self) -> f32 {
        self.fats_number
    }       
    pub fn get_protein_name(&self) -> &str {
        &self.protein_name
    }
    pub fn get_carb_name(&self) -> &str {
        &self.carb_name
    }
    pub fn get_fat_name(&self) -> &str {
        &self.fat_name
    }   
    pub fn get_protein_grams(&self) -> f32 {
        self.protein_grams
    }               
    pub fn get_carb_grams(&self) -> f32 {
        self.carb_grams
    }
    pub fn get_fat_grams(&self) -> f32 {
        self.fat_grams
    }
}

// Get the users macros, return them in a struct UserMacros
pub fn get_input_macros() -> UserMacros{
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

    let user_macros = UserMacros::new(protein, carbs, fats);
    user_macros
}

pub fn load_food_data(json_data: &str, include_all_protein: bool) -> Result<(Vec<FoodData>, Vec<FoodData>, Vec<FoodData>), serde_json::Error> {
    // Parse json data into FoodData struct and store in a vector
    // Uses ? to return an error if the json data cannot be parsed 
    // This is best for limited error handling, where error is either of the same type or only occurs once
    let mut foods: Vec<FoodData> = serde_json::from_str(json_data)?;

    // Define vectors to hold the data per each macro
    let mut proteins: Vec<FoodData> = Vec::new();
    let mut carbs: Vec<FoodData> = Vec::new();
    let mut fats: Vec<FoodData> = Vec::new();

    // Remove proteins from the food data if we are not including all protein options
    if !include_all_protein {
        // remove the food with category "Protein With Fats" from the foods vector
        foods.retain(|food| food.category != "Protein With Fats");
    }

    // Iterate through the foods vector and add the data to the corresponding vectors
    for food in foods {
        match food.category.as_str() {
            "Carbs" => carbs.push(food),
            "Fats" => fats.push(food),
            _ => proteins.push(food),
        }
    }

    // Return tuple of vectors holding structs for each macro
    Ok((proteins, carbs, fats))
}

// Select food choices, returns a tuple of structs
pub fn select_food_choices(proteins: &Vec<FoodData>, carbs: &Vec<FoodData>, fats: &Vec<FoodData>) -> (FoodData, FoodData, FoodData) {
    // Manipulate the food amunts so the macros scale to the users input
    let mut rng = rand::thread_rng();
    
    // Generate random indices for the food choices
    let protein_index = rng.gen_range(0..proteins.len() - 1);
    let carb_index = rng.gen_range(0..carbs.len() - 1);
    let fat_index = rng.gen_range(0..fats.len() - 1);

    // Clone the values when taking them from the vectors
    let protein_choice = proteins[protein_index].clone();
    let carb_choice = carbs[carb_index].clone();
    let fat_choice = fats[fat_index].clone();

    // Return a tuple of the food choices
    (protein_choice, carb_choice, fat_choice)
}

// Scale the food choices based on user macros
pub fn scale_food_choices(user_macros: &UserMacros, protein_choice: &FoodData, carb_choice: &FoodData, fat_choice: &FoodData) -> Result<ResultMacros, &'static str> {
    
    println!("Target macros - P: {}, C: {}, F: {}", 
        user_macros.get_protein(),
    user_macros.get_carbs(),
    user_macros.get_fats()
    );

    loop {
        // Define foods macros and amounts
        let mut proteins = 0.0; 
        let mut carbs = 0.0; 
        let mut fats = 0.0; 
        let mut protein_amount = 100.0;
        let mut carb_amount = 100.0;
        let mut fat_amount = 100.0;

        // Select and set protein foods values
        let scaling_protein_factor = user_macros.get_protein() / protein_choice.get_protein();

        proteins += protein_choice.get_protein() * scaling_protein_factor;
        carbs += protein_choice.get_carbs() * scaling_protein_factor;
        fats += protein_choice.get_fats() * scaling_protein_factor;
        protein_amount *= scaling_protein_factor;

        println!("After protein scaling - P: {}, C: {}, F: {}", proteins, carbs, fats);

        // Select and set carbs foods values
        let scaling_carb_factor = user_macros.get_carbs() / carb_choice.get_carbs();

        proteins += carb_choice.get_protein() * scaling_carb_factor;
        carbs += carb_choice.get_carbs() * scaling_carb_factor;
        fats += carb_choice.get_fats() * scaling_carb_factor;
        carb_amount *= scaling_carb_factor;

        println!("After carb scaling - P: {}, C: {}, F: {}", proteins, carbs, fats);

        // Check if protein surpasses the limit, if so reduce protein
        let protein_max = user_macros.get_protein() * 1.05;
        if proteins > protein_max {
            // rescale proteins with lower value
            let excess_protein = proteins - protein_max;
            let rescale_factor = excess_protein / protein_choice.get_protein();
            proteins -= protein_choice.get_protein() * rescale_factor;
            carbs -= protein_choice.get_carbs() * rescale_factor;
            fats -= protein_choice.get_fats() * rescale_factor;
            protein_amount -= 100.0 * rescale_factor;

            println!("After protein adjustment - P: {}, C: {}, F: {}", proteins, carbs, fats);
        }

        // Check if fats are too high
        let fat_max = user_macros.get_fats() * 1.05;
        let fat_min = user_macros.get_fats() * 0.95;

        println!("Fat range - Min: {}, Current: {}, Max: {}", fat_min, fats, fat_max);

        // If fats are too high, recall the loop
        if fats > fat_max {
            println!("Fats too high, ");
            return Err("Fats too high");
        }

        // If fats are too low, add fats
        if fats < fat_min {
            let excess_fat = user_macros.get_fats() - fats;
            let rescale_factor = excess_fat / fat_choice.get_fats();
            fats += fat_choice.get_fats() * rescale_factor;
            fat_amount *= rescale_factor;
        }

        // If fats are in range, procees
        if fats <= fat_max && fats >= fat_min {
            println!("Final fats phase");
            return Ok(ResultMacros {
                protein_number: proteins,
                carbs_number: carbs,
                fats_number: fats,
                protein_name: protein_choice.get_name().to_string(),
                carb_name: carb_choice.get_name().to_string(),
                fat_name: fat_choice.get_name().to_string(),
                protein_grams: protein_amount,
                carb_grams: carb_amount,
                fat_grams: fat_amount
            });
        }

        // return the error if we fail to scale the option
        return Err("Unable to scale choices");
    }
}