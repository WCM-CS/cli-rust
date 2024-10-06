
// Define the structs for storing the food, food name & FDC ID
// Define struct for user macros, proteins, fats, carbs


// Query user for macro values

// Store values in struct

// Unkown section -- Either we use conditions to determine which protein sources to load or choose at random then potentially have to select the other optioon which would require additional calls to the proxy,
// To avoid additonal calls potentially set a fat limit such that if fat macro is under x we know we must choose low fat protein option 

// Define the implimentation files to load the values into the structs per each type of food 
// Load data into each struct by category, so 4 impl functions for loading, then return as a vector of structs 
// Loading the food options into structs occurs after decision logic due to the fact that if we load all data into structs then only use 3/4 and leave one out its a memory usage error

// Based on the foods selected via random number generaton related to length of the vector store the index of the structs from their vector in another, clean up previous vectors for memory reasons

// NOTE: potential error if we chose wrong food options since going back at this point would require loading all values again aka starting over

// Once foods are selected interact with the proxy sending the data through to gather the nutrient valies of the given foods we selected


// Manipulate the food amunts so the macros scale to the users input


// Return the foods selected and their associated amounts 





fn main() {
    println!("Hello, World!");
}