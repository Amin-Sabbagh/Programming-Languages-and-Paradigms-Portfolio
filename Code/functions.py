import time

#Procedure to print health
def printHealth(playerHealth):
    print(f'''
        Your health is now {playerHealth}!
        ''')
    

#Function to generate encounter
def encounter(gameArray):
    return gameArray.pop(0)


#Function for loading screen
def loadingScreen(seconds):
    print("*******************************************************************************************************************************************")
    time.sleep(seconds)

#Function to print intro
def intro():
    print('''
        In the desolate kingdom of shadows, a lone hero dwelled in a secluded cottage with an unlikely companion – a radiant goldfish, 
        a cherished pet that brought a glimmer of warmth to the hero's otherwise despondent existence. The sinister King, ruling with unbridled malice,
        dispatched grotesque monsters to levy an unreasonable tax on the hero and ruthlessly ended the life of his beloved aquatic friend. Sworn to vengeance,
        the hero unearthed a hidden chamber containing a mystical sword, Solaceblade, 
        a weapon imbued with the power to unveil the weaknesses of its adversaries upon touch. Gripping the hilt, 
        the hero embarked on a perilous journey, fueled by grief and determination, to dismantle the oppressive regime and bring light to the shadow-laden realm.
        The epic tale of a silent protagonist and Solaceblade had just begun, with the fate of an entire kingdom hanging in the balance.
        ''')