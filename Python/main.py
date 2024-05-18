from functions import encounter, intro
from functions import printHealth
from functions import loadingScreen
from outcome import bossEnding, bossOutcome, levelOneEnding, levelOneOutcome, levelThreeEnding, levelThreeOutcome, levelTwoEnding, levelTwoOutcome


# Global variables and lists
playerHealth = 100
level1Array = ["pythonEnemy1", "pythonEnemy2","pythonEnemy3"]
level2Array = ["goEnemy1", "goEnemy2","goEnemy3"]
level3Array = ["rustEnemy1", "rustEnemy2","rustEnemy3"]
levelBossArray = ["bossEncounter1", "bossEncounter2","bossEncounter3"]


# procudure of level 1
def levelOne():
    global playerHealth
     
    
    playerHealth = int(levelOneOutcome(encounter(level1Array), playerHealth))
    loadingScreen(3)

    printHealth(playerHealth)
    loadingScreen(3)

    playerHealth = int(levelOneOutcome(encounter(level1Array), playerHealth))
    loadingScreen(3)
    
    printHealth(playerHealth)
    loadingScreen(3)

    playerHealth = int(levelOneOutcome(encounter(level1Array), playerHealth))
    loadingScreen(3)

    printHealth(playerHealth)
    loadingScreen(3)


# procudure of level 2
def levelTwo():
    global playerHealth
     
    
    playerHealth = int(levelTwoOutcome(encounter(level2Array), playerHealth))
    loadingScreen(3)

    printHealth(playerHealth)
    loadingScreen(3)

    playerHealth = int(levelTwoOutcome(encounter(level2Array), playerHealth))
    loadingScreen(3)
    
    printHealth(playerHealth)
    loadingScreen(3)

    playerHealth = int(levelTwoOutcome(encounter(level2Array), playerHealth))
    loadingScreen(3)

    printHealth(playerHealth)
    loadingScreen(3)

    
# procudure of level 3
def levelThree():
    global playerHealth
     
    
    playerHealth = int(levelThreeOutcome(encounter(level3Array), playerHealth))
    loadingScreen(3)

    printHealth(playerHealth)
    loadingScreen(3)

    playerHealth = int(levelThreeOutcome(encounter(level3Array), playerHealth))
    loadingScreen(3)
    
    printHealth(playerHealth)
    loadingScreen(3)

    playerHealth = int(levelThreeOutcome(encounter(level3Array), playerHealth))
    loadingScreen(3)

    printHealth(playerHealth)
    loadingScreen(3)

# procudure of boss
def boss():
    global playerHealth
     
    
    playerHealth = int(bossOutcome(encounter(levelBossArray), playerHealth))
    loadingScreen(3)

    printHealth(playerHealth)
    loadingScreen(3)

    playerHealth = int(bossOutcome(encounter(levelBossArray), playerHealth))
    loadingScreen(3)
    
    printHealth(playerHealth)
    loadingScreen(3)

    playerHealth = int(bossOutcome(encounter(levelBossArray), playerHealth))
    loadingScreen(3)

    printHealth(playerHealth)
    loadingScreen(3)


# Main function to start the game.
def main():
    
    print('''
            Welcome To My Game!
                ''')

    #Play Intro 
    intro()
    
    #Play Level 1
    levelOne()

    # print good or bad ending for Level 1
    # If health is lower or equal to 0 end game
    levelOneEnding(playerHealth)
    if playerHealth<=0:
        return 
    
    #Play Level 2
    levelTwo()
    
    # print good or bad ending for Level 2
    # If health is lower or equal to 0 end game
    levelTwoEnding(playerHealth)
    if playerHealth<=0:
        return 

    #Play Level 3
    levelThree()
    
    # print good or bad ending for Level 3
    # If health is lower or equal to 0 end game
    levelThreeEnding(playerHealth)
    if playerHealth<=0:
        return 
    
    #Play boss
    boss()
    
    # print endings for boss
    bossEnding(playerHealth)
    
# Entry point of the script.
if __name__ == "__main__":
    main()





