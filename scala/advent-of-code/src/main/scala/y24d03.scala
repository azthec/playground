object y24d3 {
  def execute(input: List[String] = ResourceReader.load("y24d3")): Unit = {
    val reports = parse(input)

    val (timer1, result1) = Timer.time(part1(reports))
    val (timer2, result2) = Timer.time(part2(reports))
    println(f"Elapsed time: $timer1 ms | Result: ${result1}")
    println(f"Elapsed time: $timer2 ms | Result: ${result2}")
  }

  def parse(lines: List[String]): String = lines.mkString

  def part1(lines: String): Int =
    """mul\((\d{1,3}),(\d{1,3})\)""".r
      .findAllIn(lines)
      .matchData
      .map { m =>
        (m.group(1).toInt, m.group(2).toInt)
      }
      .map { case (left: Int, right: Int) => left * right }
      .sum

  def part2(lines: String): Int = {
    val enabled = """(do\(\)|^)(.*?)(don't\(\)|$)""".r
      .findAllIn(lines)
      .matchData
      .map(_.toString)
      .toList
      .mkString
    part1(enabled)
  }
}
