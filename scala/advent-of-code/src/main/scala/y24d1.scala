object y24d1 {
  def execute(input: List[String] = ResourceReader.load("y24d1")): Unit = {
    val (left, right) = parse(input)
    require(left.length == right.length, "Lists must be of the same size")

    val (timer1, result1) = Timer.time(part1(left, right))
    val (timer2, result2) = Timer.time(part2(left, right))
    println(f"Elapsed time: $timer1 ms | Result: ${result1}")
    println(f"Elapsed time: $timer2 ms | Result: ${result2}")
  }

  def parse(lines: List[String]): (List[Int], List[Int]) = {
    lines.map { line =>
      val Array(first, second) = line.trim.split("\\s+").map(_.toInt)
      (first, second)
    }.unzip
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
