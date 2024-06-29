import unittest
from src.ex0704_binarysearch import BinarySearch

class BinarySearchTest(unittest.TestCase):
    def setUp(self):
        self.solution = BinarySearch()

    def test_empty_list(self):
        self.assertEqual(self.solution.search([], 3), -1)

    def test_single_element_found(self):
        self.assertEqual(self.solution.search([5], 5), 0)

    def test_single_element_not_found(self):
        self.assertEqual(self.solution.search([5], 3), -1)

    def test_multiple_elements_found(self):
        self.assertEqual(self.solution.search([1, 2, 3, 4, 5], 3), 2)

    def test_multiple_elements_not_found(self):
        self.assertEqual(self.solution.search([1, 2, 4, 5], 3), -1)

    def test_upper_bound(self):
        self.assertEqual(self.solution.search([1, 2, 3, 4, 5], 5), 4)

    def test_lower_bound(self):
        self.assertEqual(self.solution.search([1, 2, 3, 4, 5], 1), 0)

    def test_out_of_bounds_high(self):
        self.assertEqual(self.solution.search([1, 2, 3, 4, 5], 6), -1)

    def test_out_of_bounds_low(self):
        self.assertEqual(self.solution.search([1, 2, 3, 4, 5], 0), -1)

if __name__ == '__main__':
    unittest.main()
