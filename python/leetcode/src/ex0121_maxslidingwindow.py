from typing import List
import math

class MaxSlidingWindow:
    def maxProfit(self, prices: List[int]) -> int:
        if not prices:
            return 0

        max_profit = 0
        lowest_price = math.inf

        for price in prices:
            if (price < lowest_price):
                lowest_price = price
            potential_profit = price - lowest_price
            if (max_profit < potential_profit):
                max_profit = potential_profit
        return int(max_profit)

