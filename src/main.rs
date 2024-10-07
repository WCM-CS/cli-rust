
mod data{
    //Struct to store the food data
    struct FoodData {
        name: String,
        fdc_id: u32,
        category: String
    }

    impl FoodData {
        // Constructor 
        pub fn new(name: String, fdc_id: u32, category: String) -> Self {
            FoodData {name, fdc_id, category}
        }
    }

    // Struct to store user inputs
    pub struct UserMacros {
        protein: f32,
        carbs: f32, 
        fats: f32
    }

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
}

fn main() {
    // Define user macros
    let mut protein = String::new();
    let mut carbs = String::new();
    let mut fats = String::new();

    // Get user macros  
    println!("Enter in your desired macro nutrient values!"); 
    println!("Protein (g): ");
    std::io::stdin().read_line(&mut protein);
    println!("Carbs (g): ");
    std::io::stdin().read_line(&mut carbs);
    println!("Fats (g): "); 
    std::io::stdin().read_line(&mut fats);

    // Parse user macros
    let protein: f32 = protein.trim().parse().unwrap();
    let carbs: f32 = carbs.trim().parse().unwrap();
    let fats: f32 = fats.trim().parse().unwrap();

    // Create user macros struct
    let user_macros = data::UserMacros::new(protein, carbs, fats);

    // Print user macros
    println!("Your macros are: ");
    println!("Protein: {}", user_macros.get_protein());
    println!("Carbs: {}", user_macros.get_carbs());
    println!("Fats: {}", user_macros.get_fats());



    // If fat to protein ratio is greater than 2.5 to 23
    // so 2.5/23 = 0.1087


}
