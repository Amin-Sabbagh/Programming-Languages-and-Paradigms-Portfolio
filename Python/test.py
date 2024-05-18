import unittest
from unittest.mock import patch
from io import StringIO
from main import (
    encounter, intro, printHealth, loadingScreen,
    levelOneOutcome, levelTwoOutcome, levelThreeOutcome, bossOutcome,
    levelOneEnding, levelTwoEnding, levelThreeEnding, bossEnding, main
)

class TestGameFunctions(unittest.TestCase):

    # Level One tests

    @patch('builtins.input', return_value='1')  # Mocking user input
    def test_pythonEnemy1CorrectOutcome(self, mock_input):
      
        player_health = 0
        result = levelOneOutcome("pythonEnemy1", player_health)
        self.assertEqual(result, 50) 

    @patch('builtins.input', return_value='3')  # Mocking user input
    def test_pythonEnemy2CorrectOutcome(self, mock_input):
      
        player_health = 0
        result = levelOneOutcome("pythonEnemy2", player_health)
        self.assertEqual(result, 50) 

    @patch('builtins.input', return_value='2')  # Mocking user input
    def test_pythonEnemy3CorrectOutcome(self, mock_input):
      
        player_health = 0
        result = levelOneOutcome("pythonEnemy3", player_health)
        self.assertEqual(result, 50) 

   # Level Two tests
        
    @patch('builtins.input', return_value='2')  # Mocking user input
    def test_goEnemy1CorrectOutcome(self, mock_input):
      
        player_health = 0
        result = levelTwoOutcome("goEnemy1", player_health)
        self.assertEqual(result, 50) 

    @patch('builtins.input', return_value='1')  # Mocking user input
    def test_goEnemy2CorrectOutcome(self, mock_input):
      
        player_health = 0
        result = levelTwoOutcome("goEnemy2", player_health)
        self.assertEqual(result, 50) 

    @patch('builtins.input', return_value='1')  # Mocking user input
    def test_goEnemy3CorrectOutcome(self, mock_input):
      
        player_health = 0
        result = levelTwoOutcome("goEnemy3", player_health)
        self.assertEqual(result, 50) 


    # Level Three tests
        
    @patch('builtins.input', return_value='1')  # Mocking user input
    def test_rustEnemy1CorrectOutcome(self, mock_input):
      
        player_health = 0
        result = levelThreeOutcome("rustEnemy1", player_health)
        self.assertEqual(result, 50) 

    @patch('builtins.input', return_value='1')  # Mocking user input
    def test_rustEnemy2CorrectOutcome(self, mock_input):
      
        player_health = 0
        result = levelThreeOutcome("rustEnemy2", player_health)
        self.assertEqual(result, 50) 

    @patch('builtins.input', return_value='2')  # Mocking user input
    def test_rustEnemy3CorrectOutcome(self, mock_input):
      
        player_health = 0
        result = levelThreeOutcome("rustEnemy3", player_health)
        self.assertEqual(result, 50) 


    # Boss tests
        
    @patch('builtins.input', return_value='1')  # Mocking user input
    def test_boss1CorrectOutcome(self, mock_input):
      
        player_health = 0
        result = bossOutcome("bossEncounter1", player_health)
        self.assertEqual(result, 50) 

    @patch('builtins.input', return_value='1')  # Mocking user input
    def test_boss2CorrectOutcome(self, mock_input):
      
        player_health = 0
        result = bossOutcome("bossEncounter2", player_health)
        self.assertEqual(result, 50) 

    @patch('builtins.input', return_value='2')  # Mocking user input
    def test_boss3CorrectOutcome(self, mock_input):
      
        player_health = 0
        result = bossOutcome("bossEncounter3", player_health)
        self.assertEqual(result, 50) 

if __name__ == '__main__':
    unittest.main()