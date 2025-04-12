use std::io::stdin;

mod mod_structs;
mod weapon_structs;

fn main() {
    let mut buffer = String::new();
    for _ in 0..4 {
        let _ = stdin().read_line(&mut buffer);
        println!("back at you: {}", buffer);
        buffer.clear();
    };
}
