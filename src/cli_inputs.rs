use std::io::stdin;

pub enum UserInput {
    Full(String),
    Single(char),
    Digit(usize)
} impl UserInput {

    pub fn new(prompt: &str) -> Option<UserInput> {
        let input = Self::cli_input(prompt);
        if let Some(integer) = Self::parse_integer(&input) {
            return Some(Self::Digit(integer));
        } else if let Some(s) = input {
            return if s.len() > 1 {
                Some(Self::Full(s))
            } else {
                if let Some(c) = s.chars().nth(0) {
                    Some(Self::Single(c))
                } else {
                    None
                }
            };
        };
        None
    }

    pub fn yes_no_prompt(prompt: &str, prefer_yes: bool) -> bool {
        let ending = if prefer_yes {
            "(Y/n)?"
        } else {
            "(y/N)?"
        };
        let full_prompt = format!("{} {}", prompt, ending);
        let input = UserInput::new(&full_prompt);
        if let Some(UserInput::Single(c)) = input {
            let cl = c.to_ascii_lowercase();
            if cl == 'y' {
                return true;
            } else if cl == 'n' {
                return false;
            };
        };
        prefer_yes
    }

    pub fn looped_integer_prompt(prompt: &str, min: usize, max: usize, default_value: usize) -> usize {
        for _ in 0..5 {
            let input = UserInput::new(prompt);
            let response = if let Some(ui) = input {
                ui
            } else {
                return default_value;
            };
            match response {
                UserInput::Digit(d) => {
                    if d >= min && d <= max {
                        return d;
                    } else {
                        println!("That number exceeds the index boundary! Try again...")
                    };
                },
                _ => {
                    println!("That's not a number! Try again...");
                }
            };
        };
        default_value
    }

    pub fn riven_prompt() -> Option<Self> {
        println!("Please enter the details of your riven mod, using the stat-key below.");
        println!("C: Cold");
        println!("CC: CritChance");
        println!("CD: Crit Damage");
        println!("D: Damage");
        println!("E: Electricity");
        println!("H: Heat");
        println!("FR: Fire-Rate");
        println!("MC: Magazine Capacity");
        println!("MS: Multi-Shot");
        println!("T: Toxic");
        println!("RS: Reload Speed");
        println!("SC: Status Chance");
        println!("Some examples of valid responses:");
        println!("134 D 80 T -20 CC");
        println!("200 C -20 R");
        println!("CC 140 CD 150 D -60");
        println!("As long as you alternate between key and values, they can be in either order.");
        Self::new("Enter your key:")
    }

    fn cli_input(prompt: &str) -> Option<String> {
        let mut buffer = String::with_capacity(25);
        println!("{}", prompt);
        let _ = stdin().read_line(&mut buffer);
        buffer = String::from(buffer.trim());
        if buffer.len() > 0 {
            buffer.shrink_to_fit();
            Some(buffer)
        } else {
            None
        }
    }

    fn parse_integer(input: &Option<String>) -> Option<usize> {
        Some(input.as_ref()?.parse().ok()?)
    }

}