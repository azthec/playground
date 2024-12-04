class y24d4Test extends munit.FunSuite {
  test("should parse input") {
    val expected = List(List('M', 'M', 'M', 'S'), List('M', 'S', 'A', 'M'))
    val input: List[String] = List("MMMS", "MSAM")
    val actual = y24d4.parse(input)
    assertEquals(actual, expected)
  }

  test("should enumerate all right diagonals") {
    val expected = List(
      List('B', 'F', '2'),
      List('C', '1'),
      List('0'),
      List('A', 'E', 'I', '3'),
      List('D', 'H', 'L'),
      List('G', 'K'),
      List('J')
    )
    val input: List[String] = List(
      "ABC0",
      "DEF1",
      "GHI2",
      "JKL3"
    )
    val parsed = y24d4.parse(input)
    val actual = y24d4.diagonals(parsed)

    assertEquals(actual, expected)
  }

  test("should count valid xmas occurrences") {
    val expected = 18
    val input = List(
      "MMMSXXMASM",
      "MSAMXMSMSA",
      "AMXSXMAAMM",
      "MSAMASMSMX",
      "XMASAMXAMM",
      "XXAMMXXAMA",
      "SMSMSASXSS",
      "SAXAMASAAA",
      "MAMMMXMMMM",
      "MXMXAXMASX"
    )
    val parsed = y24d4.parse(input)
    val actual = y24d4.part1(parsed)
    assertEquals(actual, expected)
  }

  test("should count part 2 valid xmas occurrences") {
    val expected = 9
    val input = List(
      "MMMSXXMASM",
      "MSAMXMSMSA",
      "AMXSXMAAMM",
      "MSAMASMSMX",
      "XMASAMXAMM",
      "XXAMMXXAMA",
      "SMSMSASXSS",
      "SAXAMASAAA",
      "MAMMMXMMMM",
      "MXMXAXMASX"
    )
    val parsed = y24d4.parse(input)
    val actual = y24d4.part2(parsed)
    assertEquals(actual, expected)
  }
}
