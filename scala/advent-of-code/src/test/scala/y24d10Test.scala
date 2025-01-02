class y24d10Test extends munit.FunSuite {
  test("should parse input") {
    val expected =
      List(List(8, 9, 0, 1, 0, 1, 2, 3), List(7, 8, 1, 2, 1, 8, 7, 4))
    val actual = y24d10.parse(input.take(2))
    assertEquals(actual, expected)
  }

  test("should generate all possible paths") {
    val expected =
      List(Tuple2(3, 4), Tuple2(5, 4), Tuple2(4, 5), Tuple2(0, 1), Tuple2(3, 0))
    val parsed = y24d10.parse(input)
    val actual = y24d10.trail(parsed, (0, 2)).distinct
    assertEquals(actual, expected)
  }

  test("should solve part 1") {
    val expected = 36
    val parsed = y24d10.parse(input)
    val actual = y24d10.part1(parsed)
    assertEquals(actual, expected)
  }

  test("should solve part 2") {
    val expected = 81
    val parsed = y24d10.parse(input)
    val actual = y24d10.part2(parsed)
    assertEquals(actual, expected)
  }

  val input = """
    |89010123
    |78121874
    |87430965
    |96549874
    |45678903
    |32019012
    |01329801
    |10456732""".trim.stripMargin.split('\n')
}
