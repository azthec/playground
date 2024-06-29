import unittest
from src.ex0074_matrixsearch import MatrixSearch

class MatrixSearchTest(unittest.TestCase):
    def setUp(self):
        self.solution = MatrixSearch()

    def test_empty_matrix(self):
        self.assertEqual(self.solution.binarySearch([], 3), -1)

    def test_empty_matrix_rows(self):
        self.assertEqual(self.solution.binarySearch([[], []], 3), -1)

    def test_single_element_found(self):
        self.assertEqual(self.solution.binarySearch([[5]], 5), 0)

    def test_single_element_not_found(self):
        self.assertEqual(self.solution.binarySearch([[5]], 3), -1)

    def test_multiple_elements_found(self):
        self.assertEqual(self.solution.binarySearch([[1, 2, 3], [4, 5, 6]], 3), 2)

    def test_multiple_elements_not_found(self):
        self.assertEqual(self.solution.binarySearch([[1, 2], [4, 5]], 3), -1)

    def test_upper_bound(self):
        self.assertEqual(self.solution.binarySearch([[1, 2, 3], [4, 5, 6]], 5), 4)

    def test_lower_bound(self):
        self.assertEqual(self.solution.binarySearch([[1, 2, 3], [4, 5, 6]], 1), 0)

    def test_out_of_bounds_high(self):
        self.assertEqual(self.solution.binarySearch([[1, 2, 3], [4, 5, 6]], 7), -1)

    def test_out_of_bounds_low(self):
        self.assertEqual(self.solution.binarySearch([[1, 2, 3], [4, 5, 6]], 0), -1)

    def test_leetcode_case_1(self):
        self.assertEqual(self.solution.binarySearch([[1,3,5,7],[10,11,16,20],[23,30,34,60]], 3), 1)

    def test_leetcode_case_2(self):
        self.assertEqual(self.solution.binarySearch([[1,3,5,7],[10,11,16,20],[23,30,34,60]], 13), -1)


if __name__ == '__main__':
    unittest.main()
