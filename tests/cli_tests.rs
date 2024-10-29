use std::fs;
use cli_rust::food_data as FD;

#[cfg(test)]

#[test]
fn test_json_loading_all() {
    let path = "data/food_items.json";
    let json_data = fs::read_to_string(path).expect("Failed to read file");
    let (proteins_vec, carbs_vec, fats_vec) = FD::load_food_data(&json_data, true).unwrap();

    assert_eq!(proteins_vec.len(), 8);
    assert_eq!(carbs_vec.len(), 4);
    assert_eq!(fats_vec.len(), 4);
}

#[test]
fn test_json_loading_protein_filtered() {
    let path = "data/food_items.json";
    let json_data = fs::read_to_string(path).expect("Failed to read file");
    let (proteins_vec, carbs_vec, fats_vec) = FD::load_food_data(&json_data, false).unwrap();

    assert_eq!(proteins_vec.len(), 5);
    assert_eq!(carbs_vec.len(), 4);
    assert_eq!(fats_vec.len(), 4);
}