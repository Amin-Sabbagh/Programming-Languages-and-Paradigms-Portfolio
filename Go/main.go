package main

import (
	"fmt"
	"time"
)

func main() {

	// start of the game
	fmt.Println("\n\t\tWelcome to my game!")
	fmt.Println()
	fmt.Println()

	//intro
	fmt.Println("\tIn the desolate kingdom of shadows, a lone hero dwelled in a secluded cottage with an unlikely companion – a radiant goldfish, ")
	fmt.Println("\ta cherished pet that brought a glimmer of warmth to the hero's otherwise despondent existence. The sinister King, ruling with unbridled malice, ")
	fmt.Println("\tdispatched grotesque monsters to levy an unreasonable tax on the hero and ruthlessly ended the life of his beloved aquatic friend. Sworn to vengeance, ")
	fmt.Println("\tthe hero unearthed a hidden chamber containing a mystical sword, Solaceblade, ")
	fmt.Println("\ta weapon imbued with the power to unveil the weaknesses of its adversaries upon touch. Gripping the hilt, ")
	fmt.Println("\tthe hero embarked on a perilous journey, fueled by grief and determination, to dismantle the oppressive regime and bring light to the shadow-laden realm. ")
	fmt.Println("\tThe epic tale of a silent protagonist and Solaceblade had just begun, with the fate of an entire kingdom hanging in the balance.")

	// Initialize player health
	playerHealth := 100

	// Define the array of enemies for Level 1
	level1Array := []string{"pythonEnemy1", "pythonEnemy2", "pythonEnemy3"}

	// Iterate through the level 1 array
	for i := 0; i < 3; i++ {

		enemy := level1Array[i]

		switch enemy {
		case "pythonEnemy1":
			fmt.Println(
				"\n\n\n\tA normal python appears!",

				"\n\n\t'hiss! hisssssSSSS!'",

				"\n\n\tThe python attacks, but the hero dodges and hits his blade on the python's skin.",
				"\n\n\tHero's sword speaks, 'I know his weakness! Let's see how well you know your Python.",

				"\n\tWhat's the function used to remove the last element from a list?",

				"\n\n\t1. pop()",
				"\n\t2. remove()",
				"\n\t3. delete()",

				"\n\n\tAnswer correctly",
				"\n\tand the python will turn into stone!")
			println()
			fmt.Print("\tAnswer:")

			var choice string
			fmt.Scanln(&choice)
			println()

			if choice == "2" || choice == "3" {
				playerHealth -= 50
				fmt.Println("\tWRONG! And the enemy attacked! You managed to run away wounded...")
			} else if choice == "1" {
				fmt.Println("\tYou have chosen wisely, as the result the enemy turned to stone!")
				fmt.Println("\tAnd it dropped a healthy snack!")
				playerHealth += 50
			} else {
				playerHealth -= 50
				fmt.Println("\tYou ignored the swords options! And the enemy attacked! You managed to run away wounded...")
			}

		case "pythonEnemy2":
			fmt.Println(
				"\n\n\n\tThe Bundler Python appears with a bag!",

				"\n\n\t'I'm collecting booooones'",

				"\n\n\tThe python tries to grab the hero, but the hero dodges and hits his blade on the python's skin.",

				"\n\tHero's sword speaks, 'I know his weakness! Let's see how well you know your procedural paradigms.",
				"\n\tIn procedural programming, what term is used to describe the concept of bundling data and methods that operate on that data?",

				"\n\n\n\t1. Abstraction",
				"\n\t2. Polymorphism",
				"\n\t3. Encapsulation",

				"\n\n\tAnswer correctly",
				"\n\tand the python will turn into stone!")
			println()
			fmt.Print("\tAnswer:")

			var choice string
			fmt.Scanln(&choice)
			println()

			if choice == "1" || choice == "2" {
				playerHealth -= 50
				fmt.Println("\tWRONG! And the enemy attacked! You managed to run away wounded...")
			} else if choice == "3" {
				fmt.Println("\tYou have chosen wisely, as the result the enemy turned to stone!")
				fmt.Println("\tAnd it dropped a healthy snack!")
				playerHealth += 50
			} else {
				playerHealth -= 50
				fmt.Println("\tYou ignored the swords options! And the enemy attacked! You managed to run away wounded...")
			}

		case "pythonEnemy3":
			fmt.Println("\n\n\n\tThe Abstraction Python appeared!",

				"\n\n\t'Alright, Hero! I am going to swallow you whole!'",

				"\n\tThe python attacks, but the hero dodges and hits his blade on the python's skin.",
				"\n\n\tHero's sword speaks, 'I know his weakness! Let's see how well you know your Python in the context of the procedural paradigm.",
				"\n\tIn Python, what role do procedures play in the context of abstraction?",

				"\n\n\n\t1. Procedures in Python are irrelevant to abstraction and only serve as a way to organize code.",
				"\n\t2. Procedures in Python allow us to refer to multiple lines of code using a single method call, contributing to code abstraction.",
				"\n\t3. Abstraction in Python exclusively relies on object-oriented programming principles, and procedures are not involved in this process.",

				"\n\n\tAnswer correctly",
				"\n\tand the python will turn into stone!")

			fmt.Println()
			fmt.Print("\tAnswer:")

			var choice string
			fmt.Scanln(&choice)
			fmt.Println()

			if choice == "1" || choice == "3" {
				playerHealth -= 50
				fmt.Println("\tWRONG! And the enemy attacked! You managed to run away wounded...")
			} else if choice == "2" {
				fmt.Println("\tYou have chosen wisely, as the result the enemy turned to stone!")
				fmt.Println("\tAnd it dropped a healthy snack!")
				playerHealth += 50
			} else {
				playerHealth -= 50
				fmt.Println("\tYou ignored the swords options! And the enemy attacked! You managed to run away wounded...")
			}

		}
		// Loading screen
		fmt.Println("\n*******************************************************************************************************************************************\n")
		time.Sleep(3 * time.Second)
		fmt.Printf("\tYour health is now %d!\n", playerHealth)
		time.Sleep(3 * time.Second)
		fmt.Println("\n*******************************************************************************************************************************************\n")

	}

	///bad ending level 1
	if playerHealth <= 0 {
		fmt.Println("\tThe hero feels faint, all the venom from the pythons are getting him dizzy. He faints and a giant python swallows him whole.")
		fmt.Println("\tGame Over!")
		return // Use return to exit the main function and end the program
	}

	//good ending level 1
	println("\tThe hero feels Braver than ever and continues with the Journey ahead!!")

	// Define the array of enemies for Level 2
	level2Array := []string{"goEnemy1", "goEnemy2", "goEnemy3"}

	// Iterate through the level 2 array
	for i := 0; i < 3; i++ {

		enemy := level2Array[i]

		switch enemy {
		case "goEnemy1":
			fmt.Println(
				"\n\n\n\tA mischievous Gofer appears!",
				"\n\t'squeak! squeak!'",
				"\n\tThe Gofer scurries towards you, but the hero skillfully parries its movements with a swift strike of the blade.",
				"\n\tHero's sword speaks, 'I know his weakness! Let's see how well you know your Python.",
				"\n\tHero's sword speaks, 'I've deciphered its weakness! Let's test your Go language knowledge in the imperative paradigm.",
				"\n\tIn Go, what keyword is used to create a new variable?",
				"\n\n\t1. make",
				"\n\t2. var",
				"\n\t3. define",
				"\n\n\tAnswer correctly",
				"\n\tand the Gofer will scamper away in defeat!")
			println()
			fmt.Print("\tAnswer:")

			var choice string
			fmt.Scanln(&choice)
			println()

			if choice == "2" || choice == "3" {
				playerHealth -= 100
				fmt.Println("\tWRONG! And the enemy attacked! You managed to run away wounded...")
			} else if choice == "1" {
				fmt.Println("\tYou have chosen wisely, as the result the enemy ran!")
				fmt.Println("\tAnd it dropped a healthly snack!")
				playerHealth += 50
			} else {
				playerHealth -= 100
				fmt.Println("\tYou ignored the swords options! And the enemy attacked! You managed to run away wounded...")
			}

		case "goEnemy2":
			fmt.Println(
				"\n\n\n\tA cunning Gofer Mage emerges!",
				"\n\t'squeak! hiss!'",

				"\n\n\tThe Gofer Mage conjures a spell, but the hero counters with a swift incantation of the blade.",
				"\n\tHero's sword speaks, 'This one's tricky! Let's see how well you grasp Go's imperative concepts.",
				"\n\tIn Go, how do you declare and initialize a constant variable?",

				"\n\n\n\t1. const pi float32 = 3.14",
				"\n\t2. constant pi = 3.14",
				"\n\t3. pi := 3.14",
				"\n\n\tAnswer correctly",
				"\n\tand the Gofer will scamper away in defeat!")
			println()
			fmt.Print("\tAnswer:")

			var choice string
			fmt.Scanln(&choice)
			println()

			if choice == "2" || choice == "3" {
				playerHealth -= 100
				fmt.Println("\tWRONG! And the enemy attacked! You managed to run away wounded...")
			} else if choice == "1" {
				fmt.Println("\tYou have chosen wisely, as the result the enemy ran!")
				fmt.Println("\tAnd it dropped a healthly snack!")
				playerHealth += 50
			} else {
				playerHealth -= 100
				fmt.Println("\tYou ignored the swords options! And the enemy attacked! You managed to run away wounded...")
			}

		case "goEnemy3":
			fmt.Println("\n\n\n\tA formidable Gofer Commander challenges you!",
				"\n\t'squeak! roar!'",
				"\n\n\tThe Gofer Commander charges, but the hero stands firm, ready to face this formidable foe.",
				"\n\tHero's sword speaks, 'Prepare yourself! Here's an advanced Go question in the imperative paradigm.",

				"\n\tWhat is the purpose of the 'defer' keyword in Go?",

				"\n\n\n\t1. To declare a deferred function that will be executed when the surrounding function returns",
				"\n\t2. To delay the execution of a specific line of code",
				"\n\t3. To define a constant variable with deferred initialization",

				"\n\n\tAnswer correctly",
				"\n\tand the Gofer will scamper away in defeat!")
			fmt.Println()
			fmt.Print("\tAnswer:")

			var choice string
			fmt.Scanln(&choice)
			fmt.Println()

			time.Sleep(1 * time.Second)

			if choice == "2" || choice == "3" {
				playerHealth -= 100
				fmt.Println("\tWRONG! And the enemy attacked! You managed to run away wounded...")
			} else if choice == "1" {
				fmt.Println("\tYou have chosen wisely, as the result the enemy ran!")
				fmt.Println("\tAnd it dropped a healthly snack!")
				playerHealth += 50
			} else {
				playerHealth -= 100
				fmt.Println("\tYou ignored the swords options! And the enemy attacked! You managed to run away wounded...")
			}

		}
		// Loading screen
		fmt.Println("\n*******************************************************************************************************************************************\n")
		time.Sleep(3 * time.Second)
		fmt.Printf("\tYour health is now %d!\n", playerHealth)
		fmt.Println("\n*******************************************************************************************************************************************\n")
		time.Sleep(3 * time.Second)

	}

	///bad ending for level 2
	if playerHealth <= 0 {
		fmt.Println("\tThe hero feels weakened, the relentless attacks of the Gofer and its allies take their toll.",
			"\n\tOverwhelmed and battered, the hero succumbs, surrounded by mischievous Gofer creatures.",
			"\n\tGame Over!")
		return // Use return to exit the main function and end the program
	}

	//good ending level 2
	println("\tEmboldened by the successful encounter, the hero forges ahead with newfound courage!!")

	// Define the array of enemies for Level 3
	level3Array := []string{"rustEnemy1", "rustEnemy2", "rustEnemy3"}

	// Iterate through the level 3 array
	for i := 0; i < 3; i++ {

		enemy := level3Array[i]

		switch enemy {
		case "rustEnemy1":
			fmt.Println(
				"\n\n\n\tA Rusty Crab scuttles into view!",
				"\n\t'clickity-clack!'",
				"\n\n\t The Rusty Crab brandishes its claws, ready to strike, but the hero stands firm with their blade at the ready.",

				"\n\n\tHero's sword speaks, 'Time for a Rust challenge in the world of Object-Oriented programming.",
				"\n\tIn Rust, what keyword is used to define a new structure?",

				"\n\n\t1. struct",
				"\n\t2. class",
				"\n\t3. object",

				"\n\n\tAnswer correctly",
				"\n\tand the Polished Rust Crab will retreat in confusion!")
			println()
			fmt.Print("\tAnswer:")

			var choice string
			fmt.Scanln(&choice)
			println()

			if choice == "2" || choice == "3" {
				playerHealth -= 150
				fmt.Println("\tAlas, your choice was amiss! The enemy retaliates, and you narrowly escape, wounded and wary...")
			} else if choice == "1" {
				fmt.Println("\tYour knowledge prevails! The enemy is immobilized, turning to stone and leaving behind a nourishing snack!")
				playerHealth += 50
			} else {
				playerHealth -= 150
				fmt.Println("\tBeware! Ignoring the sword's guidance proves costly. The enemy strikes back, and you escape, wounded and regretful...")
			}

		case "rustEnemy2":
			fmt.Println(
				"\n\n\n\tA Polished Rust Crab emerges!",

				"\n\t'clink! clink!'",

				"\n\tThe Polished Rust Crab advances, its polished shell gleaming, but the hero prepares for a swift counterattack.",

				"\n\n\tHero's sword speaks, 'This one's a bit trickier! Let's delve deeper into Rust's OOP.",
				"\n\tHow do you define methods for a Rust struct?",

				"\n\n\n\t1. impl",
				"\n\t2. func",
				"\n\t3. method",

				"\n\n\tAnswer correctly",
				"\n\tand the Polished Rust Crab will retreat in confusion!")
			println()
			fmt.Print("\tAnswer:")

			var choice string
			fmt.Scanln(&choice)
			println()

			if choice == "2" || choice == "3" {
				playerHealth -= 150
				fmt.Println("\tAlas, your choice was amiss! The enemy retaliates, and you narrowly escape, wounded and wary...")
			} else if choice == "1" {
				fmt.Println("\tYour knowledge prevails! The enemy is immobilized, turning to stone and leaving behind a nourishing snack!")
				playerHealth += 50
			} else {
				playerHealth -= 150
				fmt.Println("\tBeware! Ignoring the sword's guidance proves costly. The enemy strikes back, and you escape, wounded and regretful...")
			}

		case "rustEnemy3":
			fmt.Println("\n\n\n\tA Rust King Crab, the mighty ruler, challenges you!",

				"\n\n\t'claw-snap! roar!'",

				"\n\n\tThe Rust King Crab prepares for a formidable battle, its massive claws ready to crush. The hero takes a defensive stance.",

				"\n\tHero's sword speaks, 'Prepare yourself for the ultimate Rust challenge in Object-Oriented programming.",
				"\n\tWhat is Rust's equivalent to inheritance in other languages?",

				"\n\n\n\t1. extend",
				"\n\t2. derive",
				"\n\t3. inherit",

				"\n\n\tAnswer correctly",
				"\n\tand the Polished Rust Crab will retreat in confusion!")
			fmt.Println()
			fmt.Print("\tAnswer:")

			var choice string
			fmt.Scanln(&choice)
			fmt.Println()

			if choice == "1" || choice == "3" {
				playerHealth -= 150
				fmt.Println("\tAlas, your choice was amiss! The enemy retaliates, and you narrowly escape, wounded and wary...")
			} else if choice == "2" {
				fmt.Println("\tYour knowledge prevails! The enemy is immobilized, turning to stone and leaving behind a nourishing snack!")
				playerHealth += 50
			} else {
				playerHealth -= 150
				fmt.Println("\tBeware! Ignoring the sword's guidance proves costly. The enemy strikes back, and you escape, wounded and regretful...")
			}

		}
		// Loading screen
		fmt.Println("\n*******************************************************************************************************************************************\n")
		time.Sleep(3 * time.Second)
		fmt.Printf("\tYour health is now %d!\n", playerHealth)
		fmt.Println("\n*******************************************************************************************************************************************\n")
		time.Sleep(3 * time.Second)
	}

	///bad ending for level 3
	if playerHealth <= 0 {
		fmt.Println("\tThe hero, weakened by the relentless assaults, succumbs to defeat. The creatures of darkness prevail.",
			"\n\tGame Over!")
		return // Use return to exit the main function and end the program
	}

	//good ending level 3
	println("\tEmpowered by your successes, the hero stands resilient, ready to face whatever challenges lie ahead.")
	println("\tOnward with courage and determination!")

	// Define the array of enemies for Level 3
	bossArray := []string{"bossEncounter1", "bossEncounter2", "bossEncounter3"}

	// Iterate through the boss array
	for i := 0; i < 3; i++ {

		enemy := bossArray[i]

		switch enemy {
		case "bossEncounter1":
			fmt.Println(
				"\n\n\n\tThe formidable King, Zephros, looms before you!",

				"\n\t'roar! thunderous footsteps!'",

				"\n\n\tZephros, the tyrant, unleashes a powerful aura. The hero stands resolute, ready to face the ultimate challenge.",

				"\n\n\tHero's sword speaks, 'Prepare for the pinnacle of programming challenges! Let's test your mastery of advanced paradigms.'",

				"\n\tWhat is the main concept behind the Actor Model in concurrent programming?",
				"\n\n\t1. Message Passing",
				"\n\t2. Thread Synchronization",
				"\n\t3. Shared Memory",
				"\n\n\tAnswer correctly,",
				"\n\tand Zephros will be shaken to his core!")
			println()
			fmt.Print("\tAnswer:")

			var choice string
			fmt.Scanln(&choice)
			println()

			if choice == "2" || choice == "3" {
				playerHealth -= 200
				fmt.Println("\tWRONG! And the king attacked! But you stand your ground...")
			} else if choice == "1" {
				fmt.Println("\tYou have chosen wisely, as a result, the king dropped a healthly snack!")
				playerHealth += 50
			} else {
				playerHealth -= 200
				fmt.Println("\tYou ignored the swords options! And the king attacked! But you stand your ground...")
			}

		case "bossEncounter2":
			fmt.Println(
				"\n\n\n\tThe tyrant Zephros retaliates with a mighty strike! But your determination holds firm, and you press on!",

				"\n\tHero's sword speaks again, 'The next challenge awaits! In functional programming, what does it mean for a function to be pure?'",

				"\n\n\n\t1. It always returns the same output for the same input",
				"\n\t2. It has no return statement",
				"\n\t3.It can modify external variables",

				"\n\n\tAnswer correctly,",
				"\n\tand Zephros will be shaken to his core!")
			println()
			fmt.Print("\tAnswer:")

			var choice string
			fmt.Scanln(&choice)
			println()

			if choice == "2" || choice == "3" {
				playerHealth -= 200
				fmt.Println("\tWRONG! And the king attacked! But you stand your ground...")
			} else if choice == "1" {
				fmt.Println("\tYou have chosen wisely, as a result, the king dropped a healthly snack!")
				playerHealth += 50
			} else {
				playerHealth -= 200
				fmt.Println("\tYou ignored the swords options! And the king attacked! But you stand your ground...")
			}

		case "bossEncounter3":
			fmt.Println("\n\n\n\tZephros, the King, reacts to your strategic prowess! But you stand tall, undeterred and ready for the final challenge!",

				"\n\tHero's sword speaks once more, 'For the grand finale! In declarative programming, what is the primary focus of the programmer?'",

				"\n\n\n\t1. Defining the step-by-step procedure",
				"\n\t2. Describing the desired outcome without specifying the control flow",
				"\n\t3. Optimizing execution speed",

				"\n\n\tAnswer correctly,",
				"\n\tand Zephros will be shaken to his core!")
			fmt.Println()
			fmt.Print("\tAnswer:")

			var choice string
			fmt.Scanln(&choice)
			fmt.Println()

			if choice == "1" || choice == "3" {
				playerHealth -= 200
				fmt.Println("\tWRONG! And the king attacked! But you stand your ground...")
			} else if choice == "2" {
				fmt.Println("\tYou have chosen wisely, as a result, the king dropped a healthly snack!")
				playerHealth += 50
			} else {
				playerHealth -= 200
				fmt.Println("\tYou ignored the swords options! And the king attacked! But you stand your ground...")
			}

		}
		// Loading screen
		fmt.Println("\n*******************************************************************************************************************************************\n")
		time.Sleep(3 * time.Second)
		fmt.Printf("\tYour health is now %d!\n", playerHealth)
		fmt.Println("\n*******************************************************************************************************************************************\n")
		time.Sleep(3 * time.Second)
	}

	//boss endings
	println()

	// Bad Ending:
	if playerHealth <= 0 {
		fmt.Println("\tDespite the hero's valiant efforts, Eldrath remained ensnared by the malevolent rule of King Zephros.")
		fmt.Println("\tThe monsters, bolstered by their tyrannical leader, crushed the hero's resistance, ")
		fmt.Println("\tleaving Solaceblade shattered and powerless. The hero, broken and defeated, witnessed the continued suffering of the kingdom. ")
		fmt.Println("\tThe memory of the lost goldfish haunted them, a symbol of the hope and joy that had been extinguished. ")
		fmt.Println("\tThe people of Eldrath languished under the oppressive regime, ")
		fmt.Println("\tand the hero's tale became a tragic reminder of the relentless cruelty that could persist in a world veiled by darkness.")
		return // Use return to exit the main function and end the program ")
	}

	// Happy Ending:
	if playerHealth > 0 && playerHealth < 250 {
		fmt.Println("\tIn the final, triumphant moments, the hero, wielding the mighty Solaceblade, rallied the oppressed citizens of Eldrath to stand against")
		fmt.Println("\tthe malevolent King and his monstrous regime. With the sword's power to expose weaknesses, the hero strategically dismantled the oppressive forces,")
		fmt.Println("\tliberating the kingdom from its shadowy grip. As sunlight bathed the once-dreaded land, the hero emerged as a symbol of resilience and justice,")
		fmt.Println("\tand Eldrath flourished anew. Rebuilding commenced, and the people, now free, celebrated their newfound liberty, and the memory of the hero's goldfish,")
		fmt.Println("\tthough lost, became a symbol of the strength that love and determination could bring.")
		return // Use return to exit the main function and end the program ")

	}

	// Amazing Ending:
	if playerHealth > 250 {
		fmt.Println("\tThe hero's journey was fraught with peril, but as they confronted the malevolent King Zephros, the true potential of Solaceblade unfolded.")
		fmt.Println("\tA dazzling display of magical prowess overwhelmed the forces of darkness, culminating in a spectacular showdown between the hero and the tyrant.")
		fmt.Println("\tIn a breathtaking climax, the hero, with Solaceblade ablaze, vanquished King Zephros, and the kingdom erupted in awe.")
		fmt.Println("\tThe hero, having earned legendary status, embarked on a new quest, leaving behind a realm forever changed by their extraordinary deeds.")
		return // Use return to exit the main function and end the program ")
	}

}
