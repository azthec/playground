object y24d2 {
  def execute(input: List[String] = ResourceReader.load("y24d2")): Unit = {
    val reports = parse(input)

    val (timer1, result1) = Timer.time(part1(reports))
    val (timer2, result2) = Timer.time(part2(reports))
    println(f"Elapsed time: $timer1 ms | Result: ${result1}")
    println(f"Elapsed time: $timer2 ms | Result: ${result2}")
  }

  def parse(lines: List[String]): List[List[Int]] = {
    lines.map(_.trim.split("\\s+").map(_.toInt).toList)
  }

  def conditionals(report: List[Int]): Boolean = {
    // sequence must be monotonically increasing or decreasing
    lazy val increasing = report.sliding(2).forall { case Seq(a, b) => a <= b }
    lazy val decreasing = report.sliding(2).forall { case Seq(a, b) => a >= b }
    // consecutive sequence elements must differ by at least one and at most three
    lazy val acceptableRanges = report.sliding(2).forall {
      case Seq(a, b) => {
        val dif = Math.abs(a - b)
        1 <= dif && dif <= 3
      }
    }

    return (increasing || decreasing) && acceptableRanges
  }

  def part1(reports: List[List[Int]]): Int = {
    reports.map(conditionals(_)).count(_ == true)
  }

  def part2(reports: List[List[Int]]): Int = {
    reports
      .map { report =>
        val combinations = (0 until report.size).map { index =>
          report.take(index) ++ report.drop(index + 1)
        }
      combinations.exists(conditionals(_))
      }
      .count(_ == true)
  }
}
