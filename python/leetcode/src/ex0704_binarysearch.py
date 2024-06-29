from typing import List

class BinarySearch:
    def search2(self, lower_bound: int, upper_bound: int, nums: List[int], target: int) -> int:
         if lower_bound > upper_bound:
             return -1
         candidate_index = (upper_bound + lower_bound) // 2
         candidate = nums[candidate_index]
         if candidate == target:
             return candidate_index
         elif candidate < target:
             return self.search2(candidate_index + 1, upper_bound, nums, target)
         else:
             return self.search2(lower_bound, candidate_index - 1, nums, target)

    def search(self, nums: List[int], target: int) -> int:
        if not nums:
            return -1
        return self.search2(0, len(nums) - 1, nums, target)
