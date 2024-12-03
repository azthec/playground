class y24d2Test extends munit.FunSuite {
  test("should parse input") {
    val expected = List(List(7, 6, 4, 2, 1), List(1, 2, 7, 8, 9))
    val input: List[String] = List("7 6 4 2 1", " 1 2 7 8 9")
    val actual = y24d2.parse(input)
    assertEquals(actual, expected)
  }

  test("should count valid reports") {
    val expected = 2
    val input: List[List[Int]] =
      List(
        List(7, 6, 4, 2, 1),
        List(1, 2, 7, 8, 9),
        List(9, 7, 6, 2, 1),
        List(1, 3, 2, 4, 5),
        List(8, 6, 4, 4, 1),
        List(1, 3, 6, 7, 9)
      )
    val actual = y24d2.part1(input)
    assertEquals(actual, expected)
  }

  test("should count part2 valid reports") {
    val expected = 4
    val input: List[List[Int]] =
      List(
        List(7, 6, 4, 2, 1),
        List(1, 2, 7, 8, 9),
        List(9, 7, 6, 2, 1),
        List(1, 3, 2, 4, 5),
        List(8, 6, 4, 4, 1),
        List(1, 3, 6, 7, 9)
      )
    val actual = y24d2.part2(input)
    assertEquals(actual, expected)
  }
}
