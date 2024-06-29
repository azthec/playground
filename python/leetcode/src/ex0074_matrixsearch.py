from typing import cast, List
from collections.abc import Sequence

from src.ex0704_binarysearch import BinarySearch

class MatrixAsList(Sequence):
    def __init__(self, matrix):
        self.matrix = matrix
        self.rows = len(matrix)
        self.cols = len(matrix[0]) if self.rows > 0 else 0

    def __len__(self):
        return self.rows * self.cols

    def __getitem__(self, index):
        if index >= self.rows * self.cols:
            raise IndexError()
        row, col = divmod(index, self.cols)
        return self.matrix[row][col]

    def __iter__(self):
        for row in self.matrix:
            for item in row:
                yield item

class MatrixSearch:
    def binarySearch(self, matrix: List[List[int]], target: int) -> int:
        bs = BinarySearch()
        return bs.search(cast(List[int], MatrixAsList(matrix)), target)

    def searchMatrix(self, matrix: List[List[int]], target: int) -> bool:
        return self.binarySearch(matrix, target) != -1

