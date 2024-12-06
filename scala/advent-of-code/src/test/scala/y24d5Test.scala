class y24d5Test extends munit.FunSuite {
  test("should parse input") {
    val expected = (
      Map(47 -> List(54), 97 -> List(13)),
      Seq(Seq(75, 47, 61, 53, 29), Seq(97, 13, 75, 29, 47))
    )
    val input: Seq[String] = Seq(
      "47|54",
      "97|13",
      "",
      "75,47,61,53,29",
      "97,13,75,29,47"
    )
    val actual = y24d5.parse(input)
    assertEquals(actual, expected)
  }

  test("should order correctly") {
    val expected =
      Seq(
        Seq(75, 47, 61, 53, 29),
        Seq(97, 61, 53, 29, 13),
        Seq(75, 29, 13),
        Seq(97, 75, 47, 61, 53),
        Seq(61, 29, 13),
        Seq(97, 75, 47, 29, 13)
      )
    val (rules, pages) = y24d5.parse(input)
    val actual = pages.map(y24d5.sort(rules, _))
    assertEquals(actual, expected)

  }

  test("should solve part 1") {
    val expected = 143
    val (rules, pages) = y24d5.parse(input)
    val actual = y24d5.part1(rules, pages)
    assertEquals(actual, expected)
  }

  test("should solve part 2") {
    val expected = 123
    val (rules, pages) = y24d5.parse(input)
    val actual = y24d5.part2(rules, pages)
    assertEquals(actual, expected)
  }

  val input = """
              |47|53
              |97|13
              |97|61
              |97|47
              |75|29
              |61|13
              |75|53
              |29|13
              |97|29
              |53|29
              |61|53
              |97|53
              |61|29
              |47|13
              |75|47
              |97|75
              |47|61
              |75|61
              |47|29
              |75|13
              |53|13
              |
              |75,47,61,53,29
              |97,61,53,29,13
              |75,29,13
              |75,97,47,61,53
              |61,13,29
              |97,13,75,29,47""".stripMargin.split("\n")
}
