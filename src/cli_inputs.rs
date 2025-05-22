use std::io::stdin;
use std::fmt::Write;

use crate::parsing::{DataLoader, ImportedGun};
use crate::weapon_structs::GunType;
use crate::gun_core::GunModdingContext;

pub fn establish_the_facts(weapon_buffer: &mut String) -> (DataLoader, usize, GunModdingContext) {
    let data = DataLoader::new(GunType::Rifle, weapon_buffer);
    let weapon_choice_index = new_weapon_select(&data.weapon_list);
    let mut modding_criteria = GunModdingContext::interview_user(
        GunType::Rifle, data.weapon_list[weapon_choice_index].get_semi()
    );
    // modding_criteria.semi = data.weapon_list[weapon_choice_index].get_semi();
    return (data, weapon_choice_index, modding_criteria);
}

pub fn new_weapon_select(imported_guns: &Vec<ImportedGun>) -> usize {
    let mut results:Vec<usize> = Vec::with_capacity(4);
    println!("Enter the weapon's name (it's case sensitive (out of spite, of course))");
    let input = take_input("Leave blank, or fuck up the input to go back:");
    for (index, gun) in imported_guns.iter().enumerate() {
        if gun.get_name() == &input {
            results.push(index)
        };
    };
    let l = results.len();
    if l < 1 {
        return new_weapon_list_select(imported_guns, &input);
    } else if l > 1 {
        return sub_weapon_select(imported_guns, &results);
    };
    results[0]
}

pub fn sub_weapon_select(imported_guns: &Vec<ImportedGun>, results: &Vec<usize>) -> usize {
    let mut buffer = String::with_capacity(300);
    _ = writeln!(buffer, "There are multiple weapons with that name:");
    for (index, &result) in results.iter().enumerate() {
        let var = imported_guns[result].get_attack();
        _ = writeln!(buffer, "{}. {}", index, var);
    };
    _ = writeln!(buffer, "Please choose the variant number you wish to optimize.");
    buffer.shrink_to_fit();
    let input = take_input(&buffer);
    results[parse_input(&input)]
}

pub fn new_weapon_list_select(imported_guns: &Vec<ImportedGun>, last_input: &str) -> usize {
    let mut buffer = String::with_capacity(1200);
    let empty = last_input == "";
    if empty {
        _ = writeln!(buffer, "You didn't input anything, or did it so badly, I couldn't tell.");
    } else {
        _ = writeln!(buffer, "Right... Couldn't find that, but I narrowed the results a bit.");
    };
    let input_first = last_input.to_uppercase().chars().next();
    for (index, weapon) in imported_guns.iter().enumerate() {
        let name = weapon.get_name();
        let attack = weapon.get_attack();
        if empty || input_first == name.chars().next() {  // checks if input is empty
            _ = writeln!(buffer, "{}. {} - {}", index, name, attack);
        };
    };
    _ = writeln!(buffer, "Please enter the number corresponding with the weapon you want to customize...");
    buffer.shrink_to_fit();
    let input = take_input(&buffer);
    parse_input(&input)
}

pub fn take_input(prompt: &str) -> String {
    let mut buffer = String::new();
    println!("{}", prompt);
    let _ = stdin().read_line(&mut buffer);
    buffer.pop();
    return buffer;
}

pub fn parse_input(input: &str) -> usize {
    if let Ok(parsed) = input.parse() {
        parsed
    } else {
        println!(
            "Error! The input'{}' could not be parsed as a number, and will now return 0.", input
        );
        0
    }
}

pub fn loop_integer_prompt(prompt: &str, min: usize, max: usize) -> usize {
    let mut curious = true;
    let mut parsed_int = 0usize;
    while curious {
        let input = take_input(prompt);
        if let Ok(parsed_input) = input.parse() {
            parsed_int = parsed_input;
        } else {
            println!("That's not a number! Try again...");
            continue;
        };
        if parsed_int > max {
            println!("That number exceeds the index boundary! Try again...")
        } else if parsed_int >= min {
            curious = false;
        } else {
            println!("I had to write an extra condition for people like you! Try again...")
        };
    };
    parsed_int
}

pub fn yes_no_prompt(prompt: &str, prefer_yes: bool) -> bool {
    let ending = if prefer_yes {
        "(Y/n)?"
    } else {
        "(y/N)?"
    };
    let full_prompt = format!("{} {}", prompt, ending);
    let input = take_input(&full_prompt);
    let lower = input.to_lowercase();
    return if lower == "y" {
        true
    } else if lower == "n" {
        false
    } else {
        prefer_yes
    };
}