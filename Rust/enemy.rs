// enemy.rs
pub trait Enemy {
    fn intro(&self);
    fn correct(&self, player_health: &mut i32);
    fn incorrect(&self, player_health: &mut i32);
    fn incorrect_input(&self, player_health: &mut i32);
    
    fn correct_choice(&self) -> i32;
   
    fn incorrect_choice_one(&self) -> i32;
    fn incorrect_choice_two(&self) -> i32;
}


// python enemies //

static PYTHON_CORRECT: &str = "\n\n\tYou have chosen wisely, as the result the enemy turned to stone!
\tAnd it dropped a healthly snack!";

static PYTHON_INCORRECT: &str = "\n\n\tWRONG! And the enemy attacked! You managed to run away wounded...";

static PYTHON_INCORRECT_INPUT: &str = "\n\n\tYou ignored the swords options! And the enemy attacked! You managed to run away wounded...";

static PYTHON_HEALTH: &i32 = &50;

pub struct PythonEnemy1;


impl Enemy for PythonEnemy1 {
    fn intro(&self) {
        print!("\tA normal python appears!

        \'hiss! hisssssSSSS!\'

        The python attacks, but the hero dodges and hits his blade on the python's skin.

        Hero\'s sword speaks, \'I know his weakness! Let\'s see how well you know your Python.
        What\'s the function used to remove the last element from a list? 

        1. pop()
        2. remove()
        3. delete()

        Answer correctly,
        and the python will turn into stone!\'");
    }

    fn correct(&self, player_health: &mut i32) {
        print!("{}", PYTHON_CORRECT);
        *player_health += PYTHON_HEALTH;
    }

    fn incorrect(&self, player_health: &mut i32) {
        print!("{}", PYTHON_INCORRECT);
        *player_health -= PYTHON_HEALTH;
    }

    fn incorrect_input(&self, player_health: &mut i32) {
        print!("{}", PYTHON_INCORRECT_INPUT);
        *player_health -= PYTHON_HEALTH;
    }

    fn correct_choice(&self) -> i32 {
       1
    }

    fn incorrect_choice_one(&self) -> i32 { 
        2
    }
    fn incorrect_choice_two(&self) -> i32 {
        3
    }
}

pub struct PythonEnemy2;

impl Enemy for PythonEnemy2 {
    fn intro(&self) {
        print!("\tThe Bundler Python appears with a bag!

        \'I'm collecting booooones\'

        The python tries to grab the hero, but the hero dodges and hits his blade on the python's skin.

        Hero\'s sword speaks, \'I know his weakness! Let\'s see how well you know your Python.
        In procedural programming, what term is used to describe the concept of bundling data and methods that operate on that data?

        1. Abstraction
        2. Polymorphism
        3. Encapsulation

        Answer correctly,
        and the python will turn into stone!\'");
    }

    fn correct(&self, player_health: &mut i32) {
        print!("{}", PYTHON_CORRECT);
        *player_health += PYTHON_HEALTH;
    }

    fn incorrect(&self, player_health: &mut i32) {
        print!("{}", PYTHON_INCORRECT);
        *player_health -= PYTHON_HEALTH;
    }

    fn incorrect_input(&self, player_health: &mut i32) {
        print!("{}", PYTHON_INCORRECT_INPUT);
        *player_health -= PYTHON_HEALTH;
    }

    fn correct_choice(&self) -> i32 {
       3
    }

    fn incorrect_choice_one(&self) -> i32 { 
        2
    }
    fn incorrect_choice_two(&self) -> i32 {
        1
    }
}

pub struct PythonEnemy3;

impl Enemy for PythonEnemy3 {
    fn intro(&self) {
        print!("\tThe Abstraction Python apeared!

        \'Alright, Hero! I am going to swallow you whole!\'

        The python attacks, but the hero dodges and hits his blade on the python's skin.

        Hero\'s sword speaks, \'I know his weakness! Let\'s see how well you know your Python in conext of the procedural paradigm.
        In Python, what role do procedures play in the context of abstraction?

        1. Procedures in Python are irrelevant to abstraction and only serve as a way to organize code.
        2. Procedures in Python allow us to refer to multiple lines of code using a single method call, contributing to code abstraction.
        3. Abstraction in Python exclusively relies on object-oriented programming principles, and procedures are not involved in this process.

        Answer correctly,
        and the python will turn into stone!\'");
    }

    fn correct(&self, player_health: &mut i32) {
        print!("{}", PYTHON_CORRECT);
        *player_health += PYTHON_HEALTH;
    }

    fn incorrect(&self, player_health: &mut i32) {
        print!("{}", PYTHON_INCORRECT);
        *player_health -= PYTHON_HEALTH;
    }

    fn incorrect_input(&self, player_health: &mut i32) {
        print!("{}", PYTHON_INCORRECT_INPUT);
        *player_health -= PYTHON_HEALTH;
    }

    fn correct_choice(&self) -> i32 {
       2
    }

    fn incorrect_choice_one(&self) -> i32 { 
        1
    }
    fn incorrect_choice_two(&self) -> i32 {
        3
    }
}

// go enemies //

static GO_CORRECT: &str = "\n\n\tYou have chosen wisely, as the result the enemy turned to stone!
\tAnd it dropped a healthly snack!";

static GO_INCORRECT: &str = "\n\n\tWRONG! And the enemy attacked! You managed to run away wounded...";

static GO_INCORRECT_INPUT: &str = "\n\n\tYou ignored the swords options! And the enemy attacked! You managed to run away wounded...";

static GO_HEALTH: &i32 = &100;

pub struct GoEnemy1;


impl Enemy for GoEnemy1 {
    fn intro(&self) {
        print!("\tA mischievous Gofer appears!

        \'squeak! squeak!\'

        The Gofer scurries towards you, but the hero skillfully parries its movements with a swift strike of the blade.

        Hero\'s sword speaks, \'I've deciphered its weakness! Let's test your Go language knowledge in the imperative paradigm.
        In Go, what keyword is used to create a new variable?

        1. make
        2. var
        3. define

        Answer correctly,
        and the Gofer will scamper away in defeat!\'");
    }

    fn correct(&self, player_health: &mut i32) {
        print!("{}", GO_CORRECT);
        *player_health += PYTHON_HEALTH;
    }

    fn incorrect(&self, player_health: &mut i32) {
        print!("{}", GO_INCORRECT);
        *player_health -= GO_HEALTH;
    }

    fn incorrect_input(&self, player_health: &mut i32) {
        print!("{}", GO_INCORRECT_INPUT);
        *player_health -= GO_HEALTH;
    }

    fn correct_choice(&self) -> i32 {
       2
    }

    fn incorrect_choice_one(&self) -> i32 { 
        1
    }
    fn incorrect_choice_two(&self) -> i32 {
        3
    }
}

pub struct GoEnemy2;

impl Enemy for GoEnemy2 {
    fn intro(&self) {
        print!("\tA cunning Gofer Mage emerges!

        \'squeak! hiss!\'

        The Gofer Mage conjures a spell, but the hero counters with a swift incantation of the blade.

        Hero\'s sword speaks, \'This one's tricky! Let's see how well you grasp Go's imperative concepts.
        In Go, how do you declare and initialize a constant variable?

        1. const pi float32 = 3.14
        2. constant pi = 3.14
        3. pi := 3.14

        Answer correctly,
        and the Gofer Mage's magic will fizzle away!\'");
    }

    fn correct(&self, player_health: &mut i32) {
        print!("{}", GO_CORRECT);
        *player_health += PYTHON_HEALTH;
    }

    fn incorrect(&self, player_health: &mut i32) {
        print!("{}", GO_INCORRECT);
        *player_health -= GO_HEALTH;
    }

    fn incorrect_input(&self, player_health: &mut i32) {
        print!("{}", GO_INCORRECT_INPUT);
        *player_health -= GO_HEALTH;
    }

    fn correct_choice(&self) -> i32 {
       1
    }

    fn incorrect_choice_one(&self) -> i32 { 
        2
    }
    fn incorrect_choice_two(&self) -> i32 {
        3
    }
}

pub struct GoEnemy3;

impl Enemy for GoEnemy3 {
    fn intro(&self) {
        print!("\tA formidable Gofer Commander challenges you!

        \'squeak! roar!\'

        The Gofer Commander charges, but the hero stands firm, ready to face this formidable foe.

        Hero\'s sword speaks, \'Prepare yourself! Here's an advanced Go question in the imperative paradigm.
        What is the purpose of the 'defer' keyword in Go?

        1. To declare a deferred function that will be executed when the surrounding function returns
        2. To delay the execution of a specific line of code
        3. To define a constant variable with deferred initialization

        Answer correctly,
        and the Gofer Commander will bow to your Go expertise!\'");
    }

    fn correct(&self, player_health: &mut i32) {
        print!("{}", GO_CORRECT);
        *player_health += PYTHON_HEALTH;
    }

    fn incorrect(&self, player_health: &mut i32) {
        print!("{}", GO_INCORRECT);
        *player_health -= GO_HEALTH;
    }

    fn incorrect_input(&self, player_health: &mut i32) {
        print!("{}", GO_INCORRECT_INPUT);
        *player_health -= GO_HEALTH;
    }

    fn correct_choice(&self) -> i32 {
       1
    }

    fn incorrect_choice_one(&self) -> i32 { 
        2
    }
    fn incorrect_choice_two(&self) -> i32 {
        3
    }
}

// rust enemies //

static RUST_CORRECT: &str = "\n\n\tYour knowledge prevails! The enemy is immobilized, turning to stone and leaving behind a nourishing snack!!";

static RUST_INCORRECT: &str = "\n\n\tAlas, your choice was amiss! The enemy retaliates, and you narrowly escape, wounded and wary...;";

static RUST_INCORRECT_INPUT: &str = "\n\n\tBeware! Ignoring the sword's guidance proves costly. The enemy strikes back, and you escape, wounded and regretful...";

static RUST_HEALTH: &i32 = &150;

pub struct RustEnemy1;


impl Enemy for RustEnemy1 {
    fn intro(&self) {
        print!("\tA Rusty Crab scuttles into view!

        \'clickity-clack!\'

        The Rusty Crab brandishes its claws, ready to strike, but the hero stands firm with their blade at the ready.

        Hero's sword speaks, \'Time for a Rust challenge in the world of Object-Oriented programming.
        In Rust, what keyword is used to define a new structure?

        1. struct
        2. class
        3. object

        Answer correctly,
        and the Rusty Crab's shell will rust away!\'");
    }

    fn correct(&self, player_health: &mut i32) {
        print!("{}", RUST_CORRECT);
        *player_health += PYTHON_HEALTH;
    }

    fn incorrect(&self, player_health: &mut i32) {
        print!("{}", RUST_INCORRECT);
        *player_health -= RUST_HEALTH;
    }

    fn incorrect_input(&self, player_health: &mut i32) {
        print!("{}", RUST_INCORRECT_INPUT);
        *player_health -= RUST_HEALTH;
    }

    fn correct_choice(&self) -> i32 {
       1
    }

    fn incorrect_choice_one(&self) -> i32 { 
        2
    }
    fn incorrect_choice_two(&self) -> i32 {
        3
    }
}

pub struct RustEnemy2;

impl Enemy for RustEnemy2 {
    fn intro(&self) {
        print!("\tA Polished Rust Crab emerges!

        \'clink! clink!\'

        The Polished Rust Crab advances, its polished shell gleaming, but the hero prepares for a swift counterattack.

        Hero's sword speaks, \'This one's a bit trickier! Let's delve deeper into Rust's OOP.
        How do you define methods for a Rust struct?

        1. impl
        2. func
        3. method

        Answer correctly,
        and the Polished Rust Crab will retreat in confusion!\'");
    }

    fn correct(&self, player_health: &mut i32) {
        print!("{}", RUST_CORRECT);
        *player_health += PYTHON_HEALTH;
    }

    fn incorrect(&self, player_health: &mut i32) {
        print!("{}", RUST_INCORRECT);
        *player_health -= RUST_HEALTH;
    }

    fn incorrect_input(&self, player_health: &mut i32) {
        print!("{}", RUST_INCORRECT_INPUT);
        *player_health -= RUST_HEALTH;
    }

    fn correct_choice(&self) -> i32 {
       1
    }

    fn incorrect_choice_one(&self) -> i32 { 
        2
    }
    fn incorrect_choice_two(&self) -> i32 {
        3
    }
}

pub struct RustEnemy3;

impl Enemy for RustEnemy3 {
    fn intro(&self) {
        print!("\tA Rust King Crab, the mighty ruler, challenges you!

        \'claw-snap! roar!\'

        The Rust King Crab prepares for a formidable battle, its massive claws ready to crush. The hero takes a defensive stance.

        Hero's sword speaks, \'Prepare yourself for the ultimate Rust challenge in Object-Oriented programming.
        What is Rust's equivalent to inheritance in other languages?

        1. extend
        2. derive
        3. inherit

        Answer correctly,
        and the Rust King Crab's reign will crumble!\'");
    }

    fn correct(&self, player_health: &mut i32) {
        print!("{}", RUST_CORRECT);
        *player_health += PYTHON_HEALTH;
    }

    fn incorrect(&self, player_health: &mut i32) {
        print!("{}", RUST_INCORRECT);
        *player_health -= RUST_HEALTH;
    }

    fn incorrect_input(&self, player_health: &mut i32) {
        print!("{}", RUST_INCORRECT_INPUT);
        *player_health -= RUST_HEALTH;
    }

    fn correct_choice(&self) -> i32 {
       2
    }

    fn incorrect_choice_one(&self) -> i32 { 
        1
    }
    fn incorrect_choice_two(&self) -> i32 {
        3
    }
}

// boss //

static BOSS_CORRECT: &str = "\n\n\tYou have chosen wisely, as a result, the king dropped a healthly snack!";

static BOSS_INCORRECT: &str = "\n\n\tWRONG! And the king attacked! But you stand your ground...";

static BOSS_INCORRECT_INPUT: &str = "\n\n\tYou ignored the swords options! And the king attacked! But you stand your ground...";

static BOSS_HEALTH: &i32 = &200;

pub struct BossEnemy1;


impl Enemy for BossEnemy1 {
    fn intro(&self) {
        print!("\tThe formidable King, Zephros, looms before you!

        \'roar! thunderous footsteps!\'

        Zephros, the tyrant, unleashes a powerful aura. The hero stands resolute, ready to face the ultimate challenge.

        Hero's sword speaks, \'Prepare for the pinnacle of programming challenges! Let's test your mastery of advanced paradigms.\'

        What is the main concept behind the Actor Model in concurrent programming?
            
        1. Message Passing
        2. Thread Synchronization
        3. Shared Memory

        Answer correctly,
        and Zephros will be shaken to his core!'");
    }

    fn correct(&self, player_health: &mut i32) {
        print!("{}", BOSS_CORRECT);
        *player_health += PYTHON_HEALTH;
    }

    fn incorrect(&self, player_health: &mut i32) {
        print!("{}", BOSS_INCORRECT);
        *player_health -= BOSS_HEALTH;
    }

    fn incorrect_input(&self, player_health: &mut i32) {
        print!("{}", BOSS_INCORRECT_INPUT);
        *player_health -= BOSS_HEALTH;
    }

    fn correct_choice(&self) -> i32 {
       1
    }

    fn incorrect_choice_one(&self) -> i32 { 
        2
    }
    fn incorrect_choice_two(&self) -> i32 {
        3
    }
}

pub struct BossEnemy2;

impl Enemy for BossEnemy2 {
    fn intro(&self) {
        print!("\tThe tyrant Zephros retaliates with a mighty strike! But your determination holds firm, and you press on!

        Hero's sword speaks again, \'The next challenge awaits! In functional programming, what does it mean for a function to be pure?\'

        1. It always returns the same output for the same input
        2. It has no return statement
        3. It can modify external variables

        Answer correctly,
        and you'll unravel Zephros's weaknesses further!");
    }

    fn correct(&self, player_health: &mut i32) {
        print!("{}", BOSS_CORRECT);
        *player_health += PYTHON_HEALTH;
    }

    fn incorrect(&self, player_health: &mut i32) {
        print!("{}", BOSS_INCORRECT);
        *player_health -= BOSS_HEALTH;
    }

    fn incorrect_input(&self, player_health: &mut i32) {
        print!("{}", BOSS_INCORRECT_INPUT);
        *player_health -= BOSS_HEALTH;
    }

    fn correct_choice(&self) -> i32 {
       1
    }

    fn incorrect_choice_one(&self) -> i32 { 
        2
    }
    fn incorrect_choice_two(&self) -> i32 {
        3
    }
}

pub struct BossEnemy3;

impl Enemy for BossEnemy3 {
    fn intro(&self) {
        print!("\tZephros, the King, reacts to your strategic prowess! But you stand tall, undeterred and ready for the final challenge!

        Hero's sword speaks once more, \'For the grand finale! In declarative programming, what is the primary focus of the programmer?\'

        1. Defining the step-by-step procedure
        2. Describing the desired outcome without specifying the control flow
        3. Optimizing execution speed

        Answer correctly,
        and the tyrant Zephros's reign will crumble!");
    }

    fn correct(&self, player_health: &mut i32) {
        print!("{}", BOSS_CORRECT);
        *player_health += PYTHON_HEALTH;
    }

    fn incorrect(&self, player_health: &mut i32) {
        print!("{}", BOSS_INCORRECT);
        *player_health -= BOSS_HEALTH;
    }

    fn incorrect_input(&self, player_health: &mut i32) {
        print!("{}", BOSS_INCORRECT_INPUT);
        *player_health -= BOSS_HEALTH;
    }

    fn correct_choice(&self) -> i32 {
       2
    }

    fn incorrect_choice_one(&self) -> i32 { 
        1
    }
    fn incorrect_choice_two(&self) -> i32 {
        3
    }
}