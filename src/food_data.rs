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
