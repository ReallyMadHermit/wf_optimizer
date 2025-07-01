use std::collections::VecDeque;
use std::io::stdin;
use std::fmt::Write;

use crate::weapon_structs::GunData;
use crate::gun_core::{DamageCriteria, ModdingContext, ReportAggregator};
use crate::file_interfacing::read_csv;

pub fn establish_the_facts() -> (GunData, ModdingContext) {
    // TODO: maybe we actually parse this buffer some??? lot of cursed splits...
    let mut gun_data_buffer = String::new();
    read_csv(&mut gun_data_buffer, "gun_data.csv");
    let selected_gun = new_weapon_select(&gun_data_buffer);
    let mut gun_modding_context = ModdingContext::interview_user(
        selected_gun.gun_type.clone(), selected_gun.semi.clone()
    );
    return (selected_gun, gun_modding_context);
}

pub fn new_weapon_select(gun_data_buffer: &str) -> GunData {
    let mut results:Vec<usize> = Vec::with_capacity(4);
    let mut csv_lines: VecDeque<&str> = gun_data_buffer.lines().collect();
    csv_lines.pop_front();
    println!("Enter the weapon's name (it's case sensitive (out of spite, of course))");
    let input = take_input("Leave blank, or fuck up the input to choose from a list:");
    for (index, &line) in csv_lines.iter().enumerate() {
        if line.split(",").collect::<Vec<_>>()[1] == input.trim() {
            results.push(index)
        };
    };
    let l = results.len();
    if l < 1 {
        return new_weapon_list_select(gun_data_buffer, &input);
    } else if l > 1 {
        return sub_weapon_select(gun_data_buffer, &results);
    };
    GunData::from_csv_line(csv_lines[results[0]])
}

pub fn sub_weapon_select(gun_data_buffer: &str, results: &Vec<usize>) -> GunData {
    let mut csv_lines: VecDeque<&str> = gun_data_buffer.lines().collect();
    csv_lines.pop_front();
    let mut buffer = String::with_capacity(300);
    _ = writeln!(buffer, "There are multiple weapons with that name:");
    for (index, &result) in results.iter().enumerate() {
        let var = csv_lines[result].split(",").collect::<Vec<_>>()[2];
        _ = writeln!(buffer, "{}. {}", index, var);
    };
    _ = writeln!(buffer, "Please choose the variant number you wish to optimize.");
    buffer.shrink_to_fit();
    let input = take_input(&buffer);
    GunData::from_csv_line(csv_lines[results[parse_input(&input.trim())]])
}

pub fn new_weapon_list_select(gun_data_buffer: &str, last_input: &str) -> GunData {
    let mut csv_lines: VecDeque<&str> = gun_data_buffer.lines().collect();
    csv_lines.pop_front();
    let mut buffer = String::with_capacity(16645);  // NOT ARBITRARY (LEN[1&2]+6)
    let empty = last_input == "";
    let input_first = last_input.to_uppercase().chars().next();
    for (index, &line) in csv_lines.iter().enumerate() {
        let split: Vec<&str> = line.split(",").collect();
        let name = split[1];
        let attack = split[2];
        if empty || input_first == name.chars().next() {  // checks if input is empty
            _ = writeln!(buffer, "{}. {} - {}", index, name, attack);
        };
    };
    _ = writeln!(buffer, "Please enter the number corresponding with the weapon you want to customize...");
    buffer.shrink_to_fit();
    let input = take_input(&buffer);
    GunData::from_csv_line(csv_lines[parse_input(&input.trim())])
}

pub fn take_input(prompt: &str) -> String {
    let mut buffer = String::new();
    println!("{}", prompt);
    let _ = stdin().read_line(&mut buffer);
    return buffer;
}

pub fn parse_input(input: &str) -> usize {
    if let Ok(parsed) = input.trim().parse() {
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
        if let Ok(parsed_input) = input.trim().parse() {
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
    let lower = input.trim().to_lowercase();
    return if lower == "y" {
        true
    } else if lower == "n" {
        false
    } else {
        prefer_yes
    };
}

pub fn results_prompt(page: usize, results: usize) {
    println!("You are now in viewing mode and may close at any time, or view other results.");
    println!("Sorting: (P) Per-shot Damage (B) Burst DPS (S) Sustained DPS");
    println!("Enter a number to change how many results are shown, or press enter to go to the next page.");
    println!("You are currently on Page {}, viewing {} results per page.", page, results);
}

pub fn cli_build_calculation_workflow() {
    let (
        selected_gun, gun_modding_context
    ) = establish_the_facts();
    let mut results = loop_integer_prompt("How many results should we display when it's done?", 0, 1000000);
    let mut page = 0;
    let mut criteria = gun_modding_context.damage;
    let report_ag = ReportAggregator::new(selected_gun, gun_modding_context);
    while true {
        report_ag.display(criteria, results, page);
        results_prompt(page, results);
        let mut little_loop = true;
        while little_loop {
            let input = take_input("What'll it be?").to_ascii_uppercase();
            let input_s = input.trim();
            match input_s {
                "P" => {
                    little_loop = false;
                    criteria = DamageCriteria::PerShot;
                    page = 0;
                },
                "B" => {
                    little_loop = false;
                    criteria = DamageCriteria::BurstDPS;
                    page = 0;
                },
                "S" => {
                    little_loop = false;
                    criteria = DamageCriteria::SustainedDPS;
                    page = 0;
                },
                "" => {
                    page += 1;
                    little_loop = false;
                },
                _ => {
                    let mut parsed_int: usize = 0;
                    if let Ok(parsed) = input_s.parse() {
                        parsed_int = parsed;
                        little_loop = false;
                        results = parsed_int;
                        page = 0;
                    } else {
                        println!("Yeah I don't know what you want...");
                    };
                }
            };
        };
    }
}