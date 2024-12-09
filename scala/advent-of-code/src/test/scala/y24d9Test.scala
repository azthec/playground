class y24d9Test extends munit.FunSuite {
  test("should parse input") {
    val expected: Seq[Int] =
      Seq(0, 0, -1, -1, -1, 1, 1, 1, -1, -1, -1, 2, -1, -1, -1, 3, 3, 3, -1, 4,
        4, -1, 5, 5, 5, 5, -1, 6, 6, 6, 6, -1, 7, 7, 7, -1, 8, 8, 8, 8, 9, 9, 10)
    val input: List[String] = List("233313312141413140201")
    val actual = y24d9.parse(input)
    assertEquals(actual, expected)
  }

  test("should tbd") {
    val expected =
      Seq(0, 0, 9, 9, 8, 1, 1, 1, 8, 8, 8, 2, 7, 7, 7, 3, 3, 3, 6, 4, 4, 6, 5,
        5, 5, 5, 6, 6, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1)
    val parsed = y24d9.parse(input)
    val actual = y24d9.testing(parsed)
    assertEquals(actual, expected)
  }

  test("should solve part 1") {
    val expected = 1928L
    val parsed = y24d9.parse(input)
    val actual = y24d9.part1(parsed)
    assertEquals(actual, expected)
  }

  // test("should solve part 2") {
  //   val expected = -1
  //   val parsed = y24d9.parse(input)
  //   val actual = y24d9.part2(parsed)
  //   assertEquals(actual, expected)
  // }

  test("should solve part1 real input") {
    val expected = 6384282079460L
    val parsed = y24d9.parse(ResourceReader.load("y24d9"))
    val actual = y24d9.part1(parsed)
    assertEquals(actual, expected)
  }

  test("should solve part2 real input") {
    val expected = 6408966547049L
    val parsed = y24d9.parse(ResourceReader.load("y24d9"))
    val actual = y24d9.part2(parsed)
    assertEquals(actual, expected)
  }

  val input = """2333133121414131402""".trim.stripMargin.split('\n')
}
