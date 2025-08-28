pub struct BuildCombo {
    pub mod_combo: [u8; 8],
    pub arcane: Option<u8>
} impl BuildCombo {
    fn new(mod_combo: [u8; 8], arcane: Option<u8>) -> Self {
        Self {mod_combo, arcane}
    }
}

pub fn generate_combinations(index_count: u8, arcane_count: u8) -> Vec<BuildCombo>  {
    let combination_count = get_combination_count(
        index_count as usize,
        8
    );
    let size = combination_count * (arcane_count + 1) as usize;
    let mut combinations: Vec<BuildCombo> = Vec::with_capacity(size);
    let mut live_array: [u8; 8] = [0, 1, 2, 3, 4, 5, 6, 6];
    for _ in 0..combination_count {
        live_array[7] = live_array[7] + 1;
        if live_array[7] == index_count {
            array_flipper(&mut live_array);
        };
        combinations.push(BuildCombo::new(live_array.clone(), None));
        for a in 0..arcane_count {
            combinations.push(BuildCombo::new(live_array.clone(), Some(a)));
        };
    };
    combinations
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
