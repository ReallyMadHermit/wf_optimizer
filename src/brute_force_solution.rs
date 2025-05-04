use crate::supporting_functions::build_mask;

// Vec<[u8;8]>
pub fn generate_combinations(index_count: u8) -> Vec<[u8;8]>  {
    let combination_count = get_combination_count(index_count as usize, 8);
    let mut combinations: Vec<[u8; 8]> = Vec::with_capacity(combination_count);
    let mut live_array: [u8; 8] = [0, 1, 2, 3, 4, 5, 6, 6];
    for _ in 0..combination_count {
        live_array[7] = live_array[7] + 1;
        if live_array[7] == index_count {
            array_flipper(&mut live_array);
        };
        combinations.push(live_array.clone());
    };
    combinations
}

fn array_flipper(array: &mut [u8; 8]) {
    let mut flip_index = 7;
    let mut top_allowed = array[flip_index] - 1;
    for i in 0..8 {
        let ia = 7-i;
        if array[ia] >= top_allowed {
            flip_index -= 1;
            top_allowed -= 1;
        };
    };
    let mut low = 0u8;
    for i in 0..8 {
        if i == flip_index {
            low = array[i] + 1;
            array[i] = low;
        } else if i > flip_index {
            low += 1;
            array[i] = low;
        };
    };
}

fn get_combination_count(unique_elements: usize, combination_length: usize) -> usize {
    if combination_length > unique_elements {
        return 0;
    };
    let mut result = 1;
    for i in 1..=combination_length {
        result = result * (unique_elements - i + 1) / i;
    };
    result
}

const ILLEGAL_PAIRS: [(usize, usize); 11] = [
    (4, 21),  // Aptitude
    (5, 27),  // Chamber
    (3, 15),  // Point Strike
    (1, 6),   // Scope
    (0, 25),  // Serration
    (16, 5),  // Acuity exclude Galvanized Chamber
    (16, 27), // Acuity exclude Split Chamber
    (16, 5),  // Acuity exclude Vigilante Armaments
    (24, 19), // Cannonade exclude Primed Shred
    (24, 26), // Cannonade exclude Speed Trigger
    (24, 31), // Cannonade exclude Vile Acceleration
];
const MAX_INDEX: usize = 35;

pub fn filter_combinations(
    combinations: &mut Vec<[u8; 8]>, required: &[usize], disallowed: &[usize]
) {
    combinations.retain(|combo: &[u8; 8]| keep_combo(combo, required, disallowed));
}

// fn keep_combo_bitmask(combo: &[u8; 8]) -> bool {
//     
// }

fn keep_combo(combo: &[u8; 8], required: &[usize], disallowed: &[usize]) -> bool {
    let mut truth_table = [false; MAX_INDEX];
    for &index in combo {
        truth_table[index as usize] = true;
    };
    for (a, b) in ILLEGAL_PAIRS {
        if truth_table[a] && truth_table[b] {
            return false;
        };
    };
    for &i in required {
        if !truth_table[i] {
            return false;
        };
    };
    for &i in disallowed {
        if truth_table[i] {
            return false;
        };
    };
    return true;
}