class y24d4Test extends munit.FunSuite {
  test("should parse input") {
    val expected = List(List('M', 'M', 'M', 'S'), List('M', 'S', 'A', 'M'))
    val input: List[String] = List("MMMS", "MSAM")
    val actual = y24d4.parse(input)
    assertEquals(actual, expected)
  }

  test("should enumerate all right diagonals") {
    val expected = List(
      List('A', 'E', 'I'),
      List('B', 'F'),
      List('C'),
      List('D', 'H'),
      List('E'),
      List('G')
    )
    val input: List[String] = List("ABC", "DEF", "GHI")
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
}
