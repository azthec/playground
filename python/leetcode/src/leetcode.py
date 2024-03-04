from __future__ import annotations
from crcmod.predefined import PredefinedCrc

class Leetcode:
    def __init__(self, base_text: str):
        self.base_text = base_text
        self.crc_calculator = PredefinedCrc("crc-ccitt-false")

