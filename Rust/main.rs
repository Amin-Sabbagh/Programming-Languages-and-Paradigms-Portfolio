// main.rs
mod enemy;
mod game;
mod player;
mod functions;

use player::Player;
use game::{Game, Level};



fn main() {
    println!("\n\tWelcome To My Game!\n");

    //print intro
    functions::intro();

    let mut player = Player::new(100);
    let mut game = Game::new();

    //play level 1
    game.play_game(&mut player.health, Level::Level1);

    //level 1 ending
    if player.health <= 0 {
        println!("\tThe hero feels faint, all the venom from the pythons are getting him dizzy. He faints and a giant python swallows him whole.
        Game Over!");
        return;
    }
    else {
        println!("\tThe hero feels Braver than ever and continues with the Journey ahead!!")
    }

    //play level 2
    game.play_game(&mut player.health, Level::Level2);

    //level 2 ending
    if player.health <= 0 {
        println!("\tThe hero feels weakened, the relentless attacks of the Gofer and its allies take their toll. 
        Overwhelmed and battered, the hero succumbs, surrounded by mischievous Gofer creatures.
        Game Over!");
        return;
    }
    else {
        println!("\tEmboldened by the successful encounter, the hero forges ahead with newfound courage!!")
    }

    //play level 3
    game.play_game(&mut player.health, Level::Level3);

    //level 3 ending
    if player.health <= 0 {
        println!("\tThe hero, weakened by the relentless assaults, succumbs to defeat. The creatures of darkness prevail.
        Game Over!");
        return;
    }
    else {
        println!("\tEmpowered by your successes, the hero stands resilient, ready to face whatever challenges lie ahead.
        Onward with courage and determination!")
    }

    //play Boss
    game.play_game(&mut player.health, Level::Boss);

    //Bad ending
    if player.health <= 0 {
        println!("\t

        Despite the hero's valiant efforts, Eldrath remained ensnared by the malevolent rule of King Zephros.
        The monsters, bolstered by their tyrannical leader, crushed the hero's resistance, 
        leaving Solaceblade shattered and powerless. The hero, broken and defeated, witnessed the continued suffering of the kingdom.
        The memory of the lost goldfish haunted them, a symbol of the hope and joy that had been extinguished.
        The people of Eldrath languished under the oppressive regime,
        and the hero's tale became a tragic reminder of the relentless cruelty that could persist in a world veiled by darkness.

    ");
        return;
    }
    
    //Happy ending
    if player.health>0 && player.health<250 {
        println!("\t
        
        In the final, triumphant moments, the hero, wielding the mighty Solaceblade, rallied the oppressed citizens of Eldrath to stand against 
        the malevolent King and his monstrous regime. With the sword's power to expose weaknesses, the hero strategically dismantled the oppressive forces,
        liberating the kingdom from its shadowy grip. As sunlight bathed the once-dreaded land, the hero emerged as a symbol of resilience and justice, 
        and Eldrath flourished anew. Rebuilding commenced, and the people, now free, celebrated their newfound liberty, and the memory of the hero's goldfish,
        though lost, became a symbol of the strength that love and determination could bring.

        ");
        return;
    }

    //Amazing Ending
    if player.health>250 {
        println!("\t

        The hero's journey was fraught with peril, but as they confronted the malevolent King Zephros, the true potential of Solaceblade unfolded. 
        A dazzling display of magical prowess overwhelmed the forces of darkness, culminating in a spectacular showdown between the hero and the tyrant.
        In a breathtaking climax, the hero, with Solaceblade ablaze, vanquished King Zephros, and the kingdom erupted in awe. 
        The hero, having earned legendary status, embarked on a new quest, leaving behind a realm forever changed by their extraordinary deeds.

        ");
        return;
    }

}