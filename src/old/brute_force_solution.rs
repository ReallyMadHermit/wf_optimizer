// // use rayon::prelude::*;
// 
// use crate::old::mod_structs::LoadedGunMods;
// 
// fn print_combo(combo: &[u8; 8], arcane: u8) {
//     println!(
//         "{}, {}, {}, {}, {}, {}, {}, {}, ac {}",
//         combo[0], combo[1], combo[2], combo[3], combo[4], combo[5], combo[6], combo[7], arcane
//     );
// }
// 
// pub fn generate_combinations(loaded_gun_mods: &LoadedGunMods) -> Vec<u64>  {
//     let index_count = loaded_gun_mods.mod_count;
//     let arcane_count = loaded_gun_mods.arcane_count;
//     let combination_count = get_combination_count(
//         index_count as usize,
//         8
//     );
//     let mut masks: Vec<u64> = Vec::with_capacity(combination_count * arcane_count as usize);
//     let mut live_array: [u8; 8] = [0, 1, 2, 3, 4, 5, 6, 6];
//     for _ in 0..combination_count {
//         live_array[7] = live_array[7] + 1;
//         if live_array[7] == index_count {
//             array_flipper(&mut live_array);
//         };
//         let mut mask = build_mask(&live_array);
//         for arcane_id in index_count..index_count + arcane_count {
//             let arcane_mask = mask | 1 << arcane_id;
//             masks.push(arcane_mask);
//         };
//     };
//     masks
// }
// 
// fn array_flipper(array: &mut [u8; 8]) {
//     let mut flip_index = 7;
//     let mut top_allowed = array[flip_index] - 1;
//     for i in 0..8 {
//         let ia = 7-i;
//         if array[ia] >= top_allowed {
//             flip_index -= 1;
//             top_allowed -= 1;
//         };
//     };
//     let mut low = 0u8;
//     for i in 0..8 {
//         if i == flip_index {
//             low = array[i] + 1;
//             array[i] = low;
//         } else if i > flip_index {
//             low += 1;
//             array[i] = low;
//         };
//     };
// }
// 
// fn get_combination_count(unique_elements: usize, combination_length: usize) -> usize {
//     if combination_length > unique_elements {
//         return 0;
//     };
//     let mut result = 1;
//     for i in 1..=combination_length {
//         result = result * (unique_elements - i + 1) / i;
//     };
//     result
// }
// 
// pub fn filter_combinations(
//     combinations: &mut Vec<u64>, required: &[u8]
// ) {
//     let required_mask = build_mask(required);
//     combinations.retain(|&combo| keep_combo_bitmask(combo, required_mask));
// }
// 
// #[inline(always)]
// fn build_mask(indices: &[u8]) -> u64 {
//     let mut mask: u64 = 0;
//     for &i in indices {
//         mask |= 1 << i;
//     };
//     mask
// }
// 
// #[inline(always)]
// fn keep_combo_bitmask(combo: u64, required_mask: u64) -> bool {
//     // create bitmask
//     // let mut bits: u64 = 0;
//     // for &i in combo.iter() {
//     //     bits |= 1 << i;
//     // };
//     
//     // // filter illegal mod pairs
//     // for (a, b) in ILLEGAL_PAIRS {
//     //     if (bits & (1 << a)) != 0 && (bits & (1 << b)) != 0 {
//     //         return false;
//     //     };
//     // };
//     
//     if (combo & required_mask) != required_mask {
//         return false;
//     };
//     
//     return true;
// }
