import unittest
from src.ex0121_maxslidingwindow import MaxSlidingWindow

class MaxSlidingWindowTest(unittest.TestCase):
    def setUp(self):
        self.solution = MaxSlidingWindow()

    def test_empty(self):
        self.assertEqual(self.solution.maxProfit([]), 0)

    def test_high_then_low(self):
        self.assertEqual(self.solution.maxProfit([2,744,1,3]), 742)

    def test_low_then_high(self):
        self.assertEqual(self.solution.maxProfit([1,743,2,300]), 742)

    def test_leetcode_case_1(self):
        self.assertEqual(self.solution.maxProfit([7,1,5,3,6,4]), 5)

    def test_leetcode_case_2(self):
        self.assertEqual(self.solution.maxProfit([7,6,4,3,1]), 0)


if __name__ == '__main__':
    unittest.main()
