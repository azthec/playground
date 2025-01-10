class y24d11Test extends munit.FunSuite {
  test("should parse input") {
    val expected = Seq(1, 2024, 1, 0, 9, 9, 2021976).map(_.toLong)
    val parsed = y24d11.parse(Seq("0 1 10 99 999"))
    assertEquals(parsed, Seq(0, 1, 10, 99, 999).map(_.toLong))

  }

  test("should count digits correctly") {
    assertEquals(y24d11.digits(1001), 4)
    assertEquals(y24d11.digits(10), 2)
    assertEquals(y24d11.digits(1), 1)
  }

  test("should split digits correctly") {
    assertEquals(y24d11.split(1001, 2), Seq(10L, 1L))
    assertEquals(y24d11.split(10, 1), Seq(1L, 0L))
  }

  test("should apply rules") {
    val expected0 = Seq(125L, 17L)
    val expected1 = Seq(253000L, 1L, 7L)
    val expected2 = Seq(253L, 0L, 2024L, 14168L)
    val expected3 = Seq(512072L, 1L, 20L, 24L, 28676032L)
    val expected4 = Seq(512L, 72L, 2024L, 2L, 0L, 2L, 4L, 2867L, 6032L)
    val expected5 = Seq(1036288L, 7L, 2L, 20L, 24L, 4048L, 1L, 4048L, 8096L, 28L, 67L, 60L, 32L)
    val expected6 = Seq(2097446912L, 14168L, 4048L, 2L, 0L, 2L, 4L, 40L, 48L, 2024L, 40L, 48L, 80L, 96L, 2L, 8L, 6L, 7L, 6L, 0L, 3L, 2L)
    assertEquals(expected0.flatMap(y24d11.rules).size, expected1.size)
    assertEquals(expected1.flatMap(y24d11.rules).size, expected2.size)
    assertEquals(expected2.flatMap(y24d11.rules).size, expected3.size)
    assertEquals(expected3.flatMap(y24d11.rules).size, expected4.size)
    assertEquals(expected4.flatMap(y24d11.rules).size, expected5.size)
    assertEquals(expected5.flatMap(y24d11.rules).size, expected6.size)
  }

  test("should solve part 1") {
    val expected = 55312L
    val parsed = Seq(125L, 17L)
    val actual = y24d11.part1(parsed)
    assertEquals(actual, expected)
  }

  test("should solve part 2") {
    val expected = 81L
    val parsed = y24d11.parse(input)
    val actual = y24d11.part2(parsed)
    assertEquals(actual, expected)
  }

  val input = Seq("0 1 10 99 999")
}
