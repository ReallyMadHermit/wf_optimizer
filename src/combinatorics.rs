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

pub fn generate_combinations(index_count: u8) -> Vec<[u8; 8]>  {
    let combination_count = get_combination_count(
        index_count as usize,
        8
    );
    let mut combinations: Vec<[u8; 8]> = Vec::with_capacity(combination_count);
    let mut live_array: [u8; 8] = [0, 1, 2, 3, 4, 5, 6, 6];
    for _ in 0..combination_count {
        live_array[7] = live_array[7] + 1;
        if live_array[7] == index_count {
            array_flipper(&mut live_array);
        };
        combinations.push(live_array.clone())
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

// [n-7, n-6, n-5, n-4, n-3, n-2, n-1, n-0]
// pub fn new_generate_combinations(index_count: u8) -> Vec<[u8; 8]> {
//     let ic = index_count as usize;
//     let combination_count = get_combination_count(ic, 8);
//     let mut combinations: Vec<[u8; 8]> = Vec::with_capacity(combination_count);
//     let mut live_array: [u8; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
//     recursive_start(
//         &mut live_array,
//         &mut combinations,
//         ic
//     );
//     combinations
// }
// 
// pub fn recursive_start(live_array: &mut [u8; 8], combinations: &mut Vec<[u8; 8]>, index_count: usize) {
//     let max = index_count - 8;
//     while live_array[0] <= max as u8 {
//         recursive_fill(live_array, combinations, index_count, 1);
//         live_array[0] += 1;
//     };
// }
// 
// pub fn recursive_fill(live_array: &mut [u8; 8], combinations: &mut Vec<[u8; 8]>, index_count: usize, depth: usize) {
//     let max = index_count - (8 - depth);
//     if depth < 7 {
//         live_array[depth] = live_array[depth - 1] + 1;
//         while live_array[depth] <= max as u8 {
//             recursive_fill(live_array, combinations, index_count, depth + 1);
//             live_array[depth] += 1;
//         }
//     } else {
//         live_array[7] = live_array[6] + 1;
//         while live_array[7] <= max as u8 {
//             combinations.push(live_array.clone());
//             live_array[7] += 1;
//         };
//     };
// }
