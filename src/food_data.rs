use std::io::Write;
use serde::{Deserialize, Serialize};
use serde_json;

//Struct to store the food data
#[derive(Debug, Deserialize, Serialize)]
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
