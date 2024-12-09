object y24d9 {
  def execute(input: Seq[String] = ResourceReader.load("y24d9")): Unit = {
    val parsed = parse(input)

    val (timer1, result1) = Timer.time(part1(parsed))
    // val (timer2, result2) = Timer.time(part2(parsed))
    println(f"Elapsed time: $timer1 ms | Result: ${result1}")
    // println(f"Elapsed time: $timer2 ms | Result: ${result2}")
  }

  def testing(lines: Seq[Int]): Seq[Int] = {
    var right = lines.size
    (for {
      left <- 0 to lines.size - 1
    } yield {
      if (right <= left)
        -1
      else if (lines(left) != -1)
        lines(left)
      else {
        right -= 1
        while (lines(right) == -1) {
          right -= 1
        }
        lines(right)
      }
    })
  }

  def part1(lines: Seq[Int]): Long = {
    var index: Long = 0L
    var total: Long = 0L
    for (element <- testing(lines)) {
      total += (if (element > 0) element * index else 0)
      index += 1
    }
    total
  }

  def part2(lines: Seq[Int]): Long = ???

  def parse(lines: Seq[String]): Seq[Int] = {
    lines.flatMap(_.map(_ - '0').zipWithIndex.flatMap { case (char, index) =>
      if (index % 2 == 0)
        Seq.fill(char)(index / 2)
      else
        Seq.fill(char)(-1)
    })
  }
}
