use std::io::stdin;

pub enum UserInput {
    Full(String),
    Single(char),
    Digit(usize)
} impl UserInput {

    pub fn new(prompt: &str) -> Option<Self> {
        let input = Self::cli_input(prompt);
        if let Some(integer) = Self::parse_integer(&input) {
            return Some(Self::Digit(integer));
        } else if let Some(s) = input {
            return if s.len() > 1 {
                Some(Self::Full(s))
            } else { s.chars().nth(0).map(Self::Single) };
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

    pub fn f32_loop(prompt: &str) -> f32 {
        loop {
            let r = Self::f32(prompt);
            if let Some(f) = r {
                return f;
            } else {
                continue;
            };
        }
    }

    pub fn f32(prompt: &str) -> Option<f32> {
        let i = Self::new(prompt);
        match i {
            Some(UserInput::Full(s)) => {
                let r = s.parse::<f32>();
                if let Ok(f) = r {
                    Some(f)
                } else {
                    return None
                }
            },
            _ => None
        }
    }

    fn cli_input(prompt: &str) -> Option<String> {
        let mut buffer = String::with_capacity(25);
        println!("{}", prompt);
        let _ = stdin().read_line(&mut buffer);
        buffer = String::from(buffer.trim());
        if !buffer.is_empty() {
            buffer.shrink_to_fit();
            Some(buffer)
        } else {
            None
        }
    }

    fn parse_integer(input: &Option<String>) -> Option<usize> {
        input.as_ref()?.parse().ok()
    }

}