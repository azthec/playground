object y24d1 {
  def execute(): Unit = {
    val input = ResourceReader.load("y24d1")
    val (left, right) = parseInput(input)
    require(left.length == right.length, "Lists must be of the same size")

    println(f"Part One: ${part1(left, right)}")
    println(f"Part Two: ${part2(left, right)}")
  }

  private def parseInput(lines: List[String]): (List[Int], List[Int]) = {
    val parsedLines = lines.map { line =>
      val Array(first, second) = line.trim.split("\\s+").map(_.toInt)
      (first, second)
    }
    parsedLines.unzip
  }

  def part1(left: List[Int], right: List[Int]): Int = {
    val pairs = left.sorted zip right.sorted
    pairs.map { case (left, right) =>
      (left - right).abs
    }.sum
  }

  def part2(left: List[Int], right: List[Int]): Int = {
    val counts: Map[Int, Int] =
      right.groupBy(identity).view.mapValues(_.size).toMap
    left.map { elem =>
      elem * counts.getOrElse(elem, 0)
    }.sum
  }
}
