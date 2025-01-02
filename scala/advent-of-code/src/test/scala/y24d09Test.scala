import y24d9.Block._
import y24d9.Block
class y24d9Test extends munit.FunSuite {
  test("should parse input") {
    val expected: Seq[Block] =
      Seq(
        File(0, 1),
        Free(2),
        File(1, 0),
        Free(3),
        File(2, 4),
        Free(5),
        File(3, 0)
      )
    val input: List[String] = List("1203450")
    val actual = y24d9.parse(input)
    assertEquals(actual, expected)
  }

  test("should solve part 1") {
    val expected = 1928L
    val parsed = y24d9.parse(input)
    val actual = y24d9.part1(parsed)
    // assertEquals(actual, expected)
  }

  val input = """2333133121414131402""".trim.stripMargin.split('\n')
}
