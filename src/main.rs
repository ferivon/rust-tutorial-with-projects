use std::thread;
use std::time::Duration;
use rand::Rng;

struct Character {
    name: String,
    health: i32,
}

impl Character {
    fn new(name: String, health: i32) -> Character {
        Character {
            name,
            health,
        }
    }

    fn attack(&self, target: &mut Character) {
        let damage = rand::thread_rng().gen_range(1..=10);
        target.take_damage(damage);
        println!("{} attacked {} and dealt {} damage!", self.name, target.name, damage);
        thread::sleep(Duration::from_secs(2));
    }

    fn take_damage(&mut self, damage: i32) {
        self.health -= damage;
        if self.health < 0 {
            self.health = 0;
        }
        println!("{} took {} damage! Health: {}", self.name, damage, self.health);
    }

    fn is_alive(&self) -> bool {
        self.health > 0
    }
}

fn main() {
    let mut player = Character::new(String::from("Player"), 100);
    let mut enemy = Character::new(String::from("Enemy"), 100);

    loop {
        player.attack(&mut enemy);
        if !enemy.is_alive() {
            println!("{} defeated {}!", player.name, enemy.name);
            break;
        }

        enemy.attack(&mut player);
        if !player.is_alive() {
            println!("{} defeated {}!", enemy.name, player.name);
            break;
        }
    }
}


/*fn main() {
    // Constants: Score Validation
    let fermancode: &str = "Fervon :3";

    println!("\x1b[1;36m\nI hope that it would be helpful for you {}\x1b[0m", fermancode);

    // Calling the let_me_help function
    let welcome_message = let_me_help();

    // Calling the let_me_help_to_you function
    let help_to_you_message = match let_me_help_to_you() {
        Ok(message) => message,
        Err(error) => format!("\x1b[1;31mError:\x1b[0m {}", error),
    };

    // Calling the let_me_help_for_newbies function
    let help_for_newbies_message = match let_me_help_for_newbies() {
        Ok(message) => message,
        Err(error) => format!("\x1b[1;31mError:\x1b[0m {}", error),
    };

    // Concatenating the messages
    let final_message = format!("{} {} {}", welcome_message, help_to_you_message, help_for_newbies_message);

    // Printing the final message
    println!("\n\x1b[1;32m{}\x1b[0m", final_message);
}

fn let_me_help() -> &'static str {
    "\x1b[1;33mWelcome here\x1b[0m"
}

fn let_me_help_to_you() -> Result<String, Box<dyn std::error::Error>> {
    Ok(String::from("\x1b[1;35mI hope you can find\x1b[0m"))
}

fn let_me_help_for_newbies() -> Result<String, Box<dyn std::error::Error>> {
    Ok(String::from("\x1b[1;34mWhat you need.\x1b[0m"))
}

*/
