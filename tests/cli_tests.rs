use std::fs;
mod foood_data;
use self::food_data as FD;

#[cfgtest]

#[test]
fn test_json_loading() {
    let path = "data/food_items.json";
    let json_data = fs::read_to_string(path).expect("Failed to read file");
    let (proteins_vec, carbs_vec, fats_vec) = FD::load_food_data(&json_data, true).unwrap();

    assert_eq!(proteins_vec.len(), 8);
    assert_eq!(carbs_vec.len(), 4);
    assert_eq!(fats_vec.len(), 4);
}