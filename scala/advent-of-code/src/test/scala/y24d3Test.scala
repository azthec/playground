class y24d3Test extends munit.FunSuite {
  test("should find and calculate mul") {
    val expected = 161
    val input = List(
      "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
    )
    val parsed = y24d3.parse(input)
    val actual = y24d3.part1(parsed)
    assertEquals(actual, expected)
  }

  test("should only mul in blocks between do and don't") {
    val expected = 48
    val input = List(
      "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
    )
    val parsed = y24d3.parse(input)
    val actual = y24d3.part2(parsed)
    assertEquals(actual, expected)
  }

  test("should start with mul enabled") {
    val expected = 8
    val input = List("xmul(2,4)&mul[3,7]!^don't")
    val parsed = y24d3.parse(input)
    val actual = y24d3.part2(parsed)
    assertEquals(actual, expected)
  }

  test("should renable mul on newline") {
    val expected = 29
    val input = List("xmul(2,4)&mul[3,7]!^don't", "mul(3,7)")
    val parsed = y24d3.parse(input)
    val actual = y24d3.part2(parsed)
    assertEquals(actual, expected)
  }

  test("should match eol as mul enabled") {
    val expected = 8
    val input = List("xmul(2,4)&mul[3,7]!")
    val parsed = y24d3.parse(input)
    val actual = y24d3.part2(parsed)
    assertEquals(actual, expected)
  }
}
