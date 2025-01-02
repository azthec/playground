import y24d8.Antenna
class y24d8Test extends munit.FunSuite {
  test("should parse input") {
    val expected = List(List('M', 'M', 'M', 'S'), List('M', 'S', 'A', 'M'))
    val input: List[String] = List("MMMS", "MSAM")
    val actual = y24d8.parse(input)
    assertEquals(actual, expected)
  }

  test("should create antenna pairs") {
    val input: Seq[Antenna] = Seq(
      Antenna('A', 5, 6),
      Antenna('A', 8, 8),
      Antenna('A', 9, 9)
    )
    val expected = List(
      (Antenna('A', 5, 6), Antenna('A', 8, 8)),
      (Antenna('A', 5, 6), Antenna('A', 9, 9)),
      (Antenna('A', 8, 8), Antenna('A', 9, 9))
    )
    val actual = y24d8.getResonantAntennas(input)
    assertEquals(actual, expected)
  }

  test("should create antinodes") {
    val input = List(
      (Antenna('A', 5, 6), Antenna('A', 8, 8)),
      (Antenna('A', 5, 6), Antenna('A', 9, 9)),
      (Antenna('A', 8, 8), Antenna('A', 9, 9))
    )
    val expected = List(
      Antenna('#', 2, 4),
      Antenna('#', 11, 10),
      Antenna('#', 1, 3),
      Antenna('#', 13, 12),
      Antenna('#', 7, 7),
      Antenna('#', 10, 10)
    )
    val actual = y24d8.antiNodes(input)
    assertEquals(actual, expected)
  }

  test("should solve part 1") {
    val expected = 14
    val parsed = y24d8.parse(input)
    val actual = y24d8.part1(parsed)
    assertEquals(actual, expected)
  }

  test("should solve part 2") {
    val expected = 34
    val parsed = y24d8.parse(input)
    val actual = y24d8.part2(parsed)
    assertEquals(actual, expected)
  }

  val input = """
    |............
    |........0...
    |.....0......
    |.......0....
    |....0.......
    |......A.....
    |............
    |............
    |........A...
    |.........A..
    |............
    |............""".trim.stripMargin.split('\n')
}
