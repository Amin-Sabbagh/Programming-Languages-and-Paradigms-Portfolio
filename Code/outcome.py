
#  level 1 #

def levelOneOutcome(encounter, playerHealth):
    
    correct ='''
        You have chosen wisely, as the result the enemy turned to stone!
        And it dropped a healthly snack!
        '''
    incorrect ='''
        WRONG! And the enemy attacked! You managed to run away wounded...
    '''
    incorrectInput ='''
        You ignored the swords options! And the enemy attacked! You managed to run away wounded...
    '''

    if encounter == "pythonEnemy1":
        dialogue = '''
        A normal python appears!

        \'hiss! hisssssSSSS!\'

        The python attacks, but the hero dodges and hits his blade on the python's skin.

        Hero\'s sword speaks, \'I know his weakness! Let\'s see how well you know your Python.
        What\'s the function used to remove the last element from a list? 

        1. pop()
        2. remove()
        3. delete()

        Answer correctly,
        and the python will turn into stone!\'

        Answer: '''   
        
        choice = input(dialogue)

        if choice == "2" or choice == "3":
           playerHealth -= 50
           print(incorrect)
           
        elif choice == "1":
            print(correct)
            playerHealth += 50
        else:
            playerHealth -= 50
            print(incorrectInput)


        return playerHealth

    if encounter == "pythonEnemy2":
        dialogue = '''
        The Bundler Python appears with a bag!
        \'I'm collecting booooones\'

        The python tries to grab the hero, but the hero dodges and hits his blade on the python's skin.

        Hero\'s sword speaks, \'I know his weakness! Let\'s see how well you know your procedural paradigms.
        In procedural programming, what term is used to describe the concept of bundling data and methods that operate on that data?

        1. Abstraction
        2. Polymorphism
        3. Encapsulation

        Answer correctly,
        and the python will turn into stone!\'

        Answer: '''
        
        choice = input(dialogue)



        if choice == "1" or choice == "2":
           playerHealth -= 50
           print(incorrect)
           
        elif choice == "3":
            print(correct)
            playerHealth += 50
        else:
            playerHealth -= 50
            print(incorrectInput)

        return playerHealth
   
    if encounter == "pythonEnemy3":
        dialogue = '''
        The Abstraction Python apeared! 
        \'Alright, Hero! I am going to swallow you whole!\'

        The python attacks, but the hero dodges and hits his blade on the python's skin.

        Hero\'s sword speaks, \'I know his weakness! Let\'s see how well you know your Python in conext of the procedural paradigm.
        In Python, what role do procedures play in the context of abstraction?

        1. Procedures in Python are irrelevant to abstraction and only serve as a way to organize code.
        2. Procedures in Python allow us to refer to multiple lines of code using a single method call, contributing to code abstraction.
        3. Abstraction in Python exclusively relies on object-oriented programming principles, and procedures are not involved in this process.

        Answer correctly,
        and the python will turn into stone!\'

        Answer: '''
        
        choice = input(dialogue)



        if choice == "1" or choice == "3":
           playerHealth -= 50
           print(incorrect)
           
        elif choice == "2":
            print(correct)
            playerHealth += 50
        else:
            playerHealth -= 50
            print(incorrectInput)

        return playerHealth
    
def levelOneEnding(playerHealth):

    if playerHealth <=0:
        print('''
                The hero feels faint, all the venom from the pythons are getting him dizzy. He faints and a giant python swallows him whole.
                Game Over!
                ''')
    else:
        print('''
                The hero feels Braver than ever and continues with the Journey ahead!!
                ''')

# level 2 #

def levelTwoOutcome(encounter, playerHealth):
    
    correct ='''
        You have chosen wisely, as the result the enemy ran!
        And it dropped a healthly snack!
        '''
    incorrect ='''
        WRONG! And the enemy attacked! You managed to run away wounded...
    '''
    incorrectInput ='''
        You ignored the swords options! And the enemy attacked! You managed to run away wounded...
    '''

    if encounter == "goEnemy1":
        dialogue = '''
            A mischievous Gofer appears!

            \'squeak! squeak!\'

            The Gofer scurries towards you, but the hero skillfully parries its movements with a swift strike of the blade.

            Hero\'s sword speaks, \'I've deciphered its weakness! Let's test your Go language knowledge in the imperative paradigm.
            In Go, what keyword is used to create a new variable?

            1. make
            2. var
            3. define

            Answer correctly,
            and the Gofer will scamper away in defeat!\'

            Answer: '''
        
        choice = input(dialogue)

        if choice == "1" or choice == "3":
           playerHealth -= 100
           print(incorrect)
           
        elif choice == "2":
            print(correct)
            playerHealth += 50
        else:
            playerHealth -= 100
            print(incorrectInput)


        return playerHealth

    if encounter == "goEnemy2":
        dialogue = '''
        A cunning Gofer Mage emerges!

        \'squeak! hiss!\'

        The Gofer Mage conjures a spell, but the hero counters with a swift incantation of the blade.

        Hero\'s sword speaks, \'This one's tricky! Let's see how well you grasp Go's imperative concepts.
        In Go, how do you declare and initialize a constant variable?

        1. const pi float32 = 3.14
        2. constant pi = 3.14
        3. pi := 3.14

        Answer correctly,
        and the Gofer Mage's magic will fizzle away!\'

        Answer: '''
        
        choice = input(dialogue)

        if choice == "2" or choice == "3":
           playerHealth -= 100
           print(incorrect)
           
        elif choice == "1":
            print(correct)
            playerHealth += 50
        else:
            playerHealth -= 100
            print(incorrectInput)

        return playerHealth
   
    if encounter == "goEnemy3":
        dialogue = '''
        A formidable Gofer Commander challenges you!

        \'squeak! roar!\'

        The Gofer Commander charges, but the hero stands firm, ready to face this formidable foe.

        Hero\'s sword speaks, \'Prepare yourself! Here's an advanced Go question in the imperative paradigm.
        What is the purpose of the 'defer' keyword in Go?

        1. To declare a deferred function that will be executed when the surrounding function returns
        2. To delay the execution of a specific line of code
        3. To define a constant variable with deferred initialization

        Answer correctly,
        and the Gofer Commander will bow to your Go expertise!\'

        Answer: '''
        
        choice = input(dialogue)



        if choice == "2" or choice == "3":
           playerHealth -= 100
           print(incorrect)
           
        elif choice == "1":
            print(correct)
            playerHealth += 50
        else:
            playerHealth -= 100
            print(incorrectInput)

        return playerHealth
       
def levelTwoEnding(playerHealth):

    if playerHealth <=0:
        print('''
                The hero feels weakened, the relentless attacks of the Gofer and its allies take their toll. 
                Overwhelmed and battered, the hero succumbs, surrounded by mischievous Gofer creatures.
                Game Over!
                ''')
    else:
        print('''
                Emboldened by the successful encounter, the hero forges ahead with newfound courage!!
                ''')

# level 3 #

def levelThreeOutcome(encounter, playerHealth):
    
    correct ='''
        Your knowledge prevails! The enemy is immobilized, turning to stone and leaving behind a nourishing snack!
        '''
    incorrect ='''
        Alas, your choice was amiss! The enemy retaliates, and you narrowly escape, wounded and wary...
    '''
    incorrectInput ='''
        Beware! Ignoring the sword's guidance proves costly. The enemy strikes back, and you escape, wounded and regretful...
    '''

    if encounter == "rustEnemy1":
        dialogue = '''
            A Rusty Crab scuttles into view!

            \'clickity-clack!\'

            The Rusty Crab brandishes its claws, ready to strike, but the hero stands firm with their blade at the ready.

            Hero's sword speaks, \'Time for a Rust challenge in the world of Object-Oriented programming.
            In Rust, what keyword is used to define a new structure?

            1. struct
            2. class
            3. object

            Answer correctly,
            and the Rusty Crab's shell will rust away!\'

            Answer:  '''   
        
        choice = input(dialogue)

        if choice == "2" or choice == "3":
           playerHealth -= 150
           print(incorrect)
           
        elif choice == "1":
            print(correct)
            playerHealth += 50
        else:
            playerHealth -= 150
            print(incorrectInput)


        return playerHealth

    if encounter == "rustEnemy2":
        dialogue = '''
        A Polished Rust Crab emerges!

        \'clink! clink!\'

        The Polished Rust Crab advances, its polished shell gleaming, but the hero prepares for a swift counterattack.

        Hero's sword speaks, \'This one's a bit trickier! Let's delve deeper into Rust's OOP.
        How do you define methods for a Rust struct?

        1. impl
        2. func
        3. method

        Answer correctly,
        and the Polished Rust Crab will retreat in confusion!\'

        Answer: '''
        
        choice = input(dialogue)



        if choice == "2" or choice == "3":
           playerHealth -= 150
           print(incorrect)
           
        elif choice == "1":
            print(correct)
            playerHealth += 50
        else:
            playerHealth -= 150
            print(incorrectInput)

        return playerHealth
   
    if encounter == "rustEnemy3":
        dialogue = '''
        A Rust King Crab, the mighty ruler, challenges you!

        \'claw-snap! roar!\'

        The Rust King Crab prepares for a formidable battle, its massive claws ready to crush. The hero takes a defensive stance.

        Hero's sword speaks, \'Prepare yourself for the ultimate Rust challenge in Object-Oriented programming.
        What is Rust's equivalent to inheritance in other languages?

        1. extend
        2. derive
        3. inherit

        Answer correctly,
        and the Rust King Crab's reign will crumble!\'

        Answer: '''
        
        choice = input(dialogue)



        if choice == "1" or choice == "3":
           playerHealth -= 150
           print(incorrect)
           
        elif choice == "2":
            print(correct)
            playerHealth += 50
        else:
            playerHealth -= 150
            print(incorrectInput)

        return playerHealth
       
def levelThreeEnding(playerHealth):

    if playerHealth <=0:
        print('''
                The hero, weakened by the relentless assaults, succumbs to defeat. The creatures of darkness prevail.
                Game Over!
                ''')
    else:
        print('''
                Empowered by your successes, the hero stands resilient, ready to face whatever challenges lie ahead.
                Onward with courage and determination!
                ''')

# Boss #

def bossOutcome(encounter, playerHealth):
    
    correct ='''
        You have chosen wisely, as a result, the king dropped a healthly snack!
        '''
    incorrect ='''
        WRONG! And the king attacked! But you stand your ground...
    '''
    incorrectInput ='''
        You ignored the swords options! And the king attacked! But you stand your ground...
    '''

    if encounter == "bossEncounter1":
        dialogue = '''
        The formidable King, Zephros, looms before you!

        \'roar! thunderous footsteps!\'

        Zephros, the tyrant, unleashes a powerful aura. The hero stands resolute, ready to face the ultimate challenge.

        Hero's sword speaks, \'Prepare for the pinnacle of programming challenges! Let's test your mastery of advanced paradigms.\'

        What is the main concept behind the Actor Model in concurrent programming?
            
        1. Message Passing
        2. Thread Synchronization
        3. Shared Memory

        Answer correctly,
        and Zephros will be shaken to his core!

        Answer: '''   
        
        choice = input(dialogue)

        if choice == "2" or choice == "3":
           playerHealth -= 200
           print(incorrect)
           
        elif choice == "1":
            print(correct)
            playerHealth += 50
        else:
            playerHealth -= 200
            print(incorrectInput)


        return playerHealth

    if encounter == "bossEncounter2":
        dialogue = '''
        The tyrant Zephros retaliates with a mighty strike! But your determination holds firm, and you press on!

        Hero's sword speaks again, \'The next challenge awaits! In functional programming, what does it mean for a function to be pure?\'

        1. It always returns the same output for the same input
        2. It has no return statement
        3. It can modify external variables

        Answer correctly,
        and you'll unravel Zephros's weaknesses further!

        Answer: '''
        
        choice = input(dialogue)



        if choice == "2" or choice == "3":
           playerHealth -= 250
           print(incorrect)
           
        elif choice == "1":
            print(correct)
            playerHealth += 50
        else:
            playerHealth -= 250
            print(incorrectInput)

        return playerHealth
   
    if encounter == "bossEncounter3":
        dialogue = '''
        Zephros, the King, reacts to your strategic prowess! But you stand tall, undeterred and ready for the final challenge!

        Hero's sword speaks once more, \'For the grand finale! In declarative programming, what is the primary focus of the programmer?\'

        1. Defining the step-by-step procedure
        2. Describing the desired outcome without specifying the control flow
        3. Optimizing execution speed

        Answer correctly,
        and the tyrant Zephros's reign will crumble!

        Answer: '''
        
        choice = input(dialogue)



        if choice == "1" or choice == "3":
           playerHealth -= 300
           print(incorrect)
           
        elif choice == "2":
            print(correct)
            playerHealth += 50
        else:
            playerHealth -= 300
            print(incorrectInput)

        return playerHealth
         
def bossEnding(playerHealth):


    # Bad Ending: #
    if playerHealth <=0:
        print('''
                Despite the hero's valiant efforts, Eldrath remained ensnared by the malevolent rule of King Zephros.
                The monsters, bolstered by their tyrannical leader, crushed the hero's resistance, 
                leaving Solaceblade shattered and powerless. The hero, broken and defeated, witnessed the continued suffering of the kingdom.
                The memory of the lost goldfish haunted them, a symbol of the hope and joy that had been extinguished.
                The people of Eldrath languished under the oppressive regime,
                and the hero's tale became a tragic reminder of the relentless cruelty that could persist in a world veiled by darkness.

                ''')
    # Happy Ending: #
    if playerHealth>0 and playerHealth<250:
        print('''
                In the final, triumphant moments, the hero, wielding the mighty Solaceblade, rallied the oppressed citizens of Eldrath to stand against 
                the malevolent King and his monstrous regime. With the sword's power to expose weaknesses, the hero strategically dismantled the oppressive forces,
                liberating the kingdom from its shadowy grip. As sunlight bathed the once-dreaded land, the hero emerged as a symbol of resilience and justice, 
                and Eldrath flourished anew. Rebuilding commenced, and the people, now free, celebrated their newfound liberty, and the memory of the hero's goldfish,
                 though lost, became a symbol of the strength that love and determination could bring.

                ''')
        
    # Amazing Ending: #
    if playerHealth>250:
        print('''
                The hero's journey was fraught with peril, but as they confronted the malevolent King Zephros, the true potential of Solaceblade unfolded. 
                A dazzling display of magical prowess overwhelmed the forces of darkness, culminating in a spectacular showdown between the hero and the tyrant.
                In a breathtaking climax, the hero, with Solaceblade ablaze, vanquished King Zephros, and the kingdom erupted in awe. 
                The hero, having earned legendary status, embarked on a new quest, leaving behind a realm forever changed by their extraordinary deeds.

                ''')




