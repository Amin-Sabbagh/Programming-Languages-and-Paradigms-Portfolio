// game.rs

use crate::functions;
use crate::enemy::{Enemy, PythonEnemy1, PythonEnemy2, PythonEnemy3,
                    GoEnemy1, GoEnemy2, GoEnemy3,
                    RustEnemy1, RustEnemy2, RustEnemy3,
                    BossEnemy1, BossEnemy2, BossEnemy3};
use std::io::{self, Write};
use std::convert::TryInto;


pub enum Level {
    Level1,
    Level2,
    Level3,
    Boss,
}

pub struct Game;

impl Game {
    pub fn new() -> Self {
        Game
    }

    pub fn play_game(&mut self, player_health: &mut i32, level: Level) {
        match level {
            Level::Level1 => {
                
                let pe1: Box<dyn Enemy> = Box::new(PythonEnemy1);
              

                self.play_with_enemy(&pe1, player_health);
                
                functions::loading();

                println!("\n\n\tYour health is now {}\n", player_health);

                functions::loading();

                let pe2: Box<dyn Enemy> = Box::new(PythonEnemy2);
              

                self.play_with_enemy(&pe2, player_health);
                
                functions::loading();

                println!("\n\n\tYour health is now {}\n", player_health);

                functions::loading();

                let pe3: Box<dyn Enemy> = Box::new(PythonEnemy3);
              

                self.play_with_enemy(&pe3, player_health);
                
                functions::loading();

                println!("\n\n\tYour health is now {}\n", player_health);

                functions::loading();

              
            }

            Level::Level2 => {
                
                let ge1: Box<dyn Enemy> = Box::new(GoEnemy1);
              

                self.play_with_enemy(&ge1, player_health);
                
                functions::loading();

                println!("\n\n\tYour health is now {}\n", player_health);

                functions::loading();

                let ge2: Box<dyn Enemy> = Box::new(GoEnemy2);
              

                self.play_with_enemy(&ge2, player_health);
                
                functions::loading();

                println!("\n\n\tYour health is now {}\n", player_health);

                functions::loading();

                let ge3: Box<dyn Enemy> = Box::new(GoEnemy3);
              

                self.play_with_enemy(&ge3, player_health);
                
                functions::loading();

                println!("\n\n\tYour health is now {}\n", player_health);

                functions::loading();

              
            }

            Level::Level3 => {
                
                let re1: Box<dyn Enemy> = Box::new(RustEnemy1);
              

                self.play_with_enemy(&re1, player_health);
                
                functions::loading();

                println!("\n\n\tYour health is now {}\n", player_health);

                functions::loading();

                let re2: Box<dyn Enemy> = Box::new(RustEnemy2);
              

                self.play_with_enemy(&re2, player_health);
                
                functions::loading();

                println!("\n\n\tYour health is now {}\n", player_health);

                functions::loading();

                let re3: Box<dyn Enemy> = Box::new(RustEnemy3);
              

                self.play_with_enemy(&re3, player_health);
                
                functions::loading();

                println!("\n\n\tYour health is now {}\n", player_health);

                functions::loading();

              
            }

            Level::Boss => {
                
                let b1: Box<dyn Enemy> = Box::new(BossEnemy1);
              

                self.play_with_enemy(&b1, player_health);
                
                functions::loading();

                println!("\n\n\tYour health is now {}\n", player_health);

                functions::loading();

                let b2: Box<dyn Enemy> = Box::new(BossEnemy2);
              

                self.play_with_enemy(&b2, player_health);
                
                functions::loading();

                println!("\n\n\tYour health is now {}\n", player_health);

                functions::loading();

                let b3: Box<dyn Enemy> = Box::new(BossEnemy3);
              

                self.play_with_enemy(&b3, player_health);
                
                functions::loading();

                println!("\n\n\tYour health is now {}\n", player_health);

                functions::loading();

              
            }
        }
    }

    fn play_with_enemy(&self, enemy: &Box<dyn Enemy>, player_health: &mut i32) {
        enemy.intro();
        
         
        

        // Get player's choice
        let player_choice = self.get_player_choice();

        // Process player's choice
        match player_choice {
            choice if choice == enemy.correct_choice().try_into().unwrap() => {
                enemy.correct(player_health);
            }
            choice if choice == enemy.incorrect_choice_one().try_into().unwrap() => {
                enemy.incorrect(player_health);
            }
            choice if choice == enemy.incorrect_choice_two().try_into().unwrap() => {
                enemy.incorrect(player_health);
            }
            _ => {
                enemy.incorrect_input(player_health);
            }
        }
    }

    fn get_player_choice(&self) -> u32 {
        loop {
            print!("\n\n\n\tAnswer: ");
            io::stdout().flush().expect("Failed to flush stdout");
    
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
    
            match input.trim().parse() {
                Ok(choice) => return choice,
                Err(_) => println!("Invalid input! Please enter a number."),
            }
        }
    }
}

