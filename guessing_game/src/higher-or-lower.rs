use std::io;
use rand::Rng;

struct Game {
    current_number: i32,
    score: i32,
    streak: i32,
    best_streak: i32,
    range_min: i32,
    range_max: i32,
}

impl Game {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        Game {
            current_number: rng.gen_range(1..=100),
            score: 0,
            streak: 0,
            best_streak: 0,
            range_min: 1,
            range_max: 100,
        }
    }

    fn display_current_state(&self) {
        println!("\n{}", "=".repeat(50));
        println!("HIGHER OR LOWER GAME");
        println!("{}", "=".repeat(50));
        println!("Current Number: {}", self.current_number);
        println!("Score: {} | Streak: {} | Best Streak: {}", 
                 self.score, self.streak, self.best_streak);
        println!("Range: {} - {}", self.range_min, self.range_max);
        println!("{}", "-".repeat(50));
    }

    fn get_user_guess(&self) -> char {
        loop {
            println!("Will the next number be (h)igher or (l)ower? (q to quit)");
            print!("> ");
            
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read input");
            
            match input.trim().to_lowercase().chars().next() {
                Some('h') => return 'h',
                Some('l') => return 'l',
                Some('q') => return 'q',
                _ => println!("Invalid input, Please enter 'h' for higher, 'l' for lower, or 'q' to quit."),
            }
        }
    }

    fn generate_next_number(&self) -> i32 {
        let mut rng = rand::thread_rng();
        
        // Generate a number that's not the same as current
        loop {
            let next = rng.gen_range(self.range_min..=self.range_max);
            if next != self.current_number {
                return next;
            }
        }
    }

    fn check_guess(&mut self, guess: char, next_number: i32) -> bool {
        let is_higher = next_number > self.current_number;
        let is_correct = (guess == 'h' && is_higher) || (guess == 'l' && !is_higher);
        
        println!("\nThe next number is: {}", next_number);
        
        if is_correct {
            self.score += 1;
            self.streak += 1;
            if self.streak > self.best_streak {
                self.best_streak = self.streak;
            }
            println!("Correct! Well done!");
            if self.streak > 1 {
                println!("🔥 You're on a {} streak!", self.streak);
            }
        } else {
            println!("Wrong! The number was {} than {}.", 
                     if is_higher { "higher" } else { "lower" }, 
                     self.current_number);
            if self.streak > 0 {
                println!("Streak broken at {}!", self.streak);
            }
            self.streak = 0;
        }
        
        self.current_number = next_number;
        is_correct
    }

    fn show_final_stats(&self) {
        println!("\n{}", "=".repeat(50));
        println!("🏁 GAME OVER - FINAL STATS 🏁");
        println!("{}", "=".repeat(50));
        println!("Final Score: {}", self.score);
        println!("Best Streak: {}", self.best_streak);
        
        let performance = match self.score {
            0..=5 => "Getting started!",
            6..=15 => "Not bad!",
            16..=25 => "Good job!",
            26..=35 => "Excellent!",
            _ => "LEGENDARY!",
        };
        
        println!("Performance: {}", performance);
        println!("Thanks for playing!");
        println!("{}", "=".repeat(50));
    }

    fn play(&mut self) {
        println!("Welcome to Higher or Lower!");
        println!("I'll show you a number, and you guess if the next one will be higher or lower.");
        println!("Numbers range from {} to {}. Let's begin!\n", self.range_min, self.range_max);

        loop {
            self.display_current_state();
            
            let guess = self.get_user_guess();
            
            if guess == 'q' {
                break;
            }
            
            let next_number = self.generate_next_number();
            self.check_guess(guess, next_number);
            
            // Small pause for dramatic effect
            println!("\nPress Enter to continue...");
            let mut pause = String::new();
            io::stdin().read_line(&mut pause).ok();
        }
        
        self.show_final_stats();
    }
}

fn main() {
    let mut game = Game::new();
    game.play();
}