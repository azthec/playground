object y24d4 {
  def execute(input: List[String] = ResourceReader.load("y24d4")): Unit = {
    val reports = parse(input)

    val (timer1, result1) = Timer.time(part1(reports))
    val (timer2, result2) = Timer.time(part2(reports))
    println(f"Elapsed time: $timer1 ms | Result: ${result1}")
    println(f"Elapsed time: $timer2 ms | Result: ${result2}")
  }

  def parse(lines: List[String]): List[List[Char]] = lines.map(_.toList)

  def counts(lines: List[List[Char]]): Int = lines.map(count(_)).sum

  def count(line: List[Char]): Int = {
    line
      .sliding(4)
      .collect { case List(a, b, c, d) =>
        (a, b, c, d) // ignore any smaller than 4
      }
      .map {
        _ match {
          case Tuple4('X', 'M', 'A', 'S') => true
          case Tuple4('S', 'A', 'M', 'X') => true
          case _                          => false
        }
      }
      .count(_ == true)
  }

  def diagonals(lines: List[List[Char]]): List[List[Char]] = {
    (for {
      i <- lines.indices
      j <- lines.indices
      if i + j < lines.size
    } yield {
      (for {
        k <- 0 until lines.size - i - j
      } yield lines(i + k)(j + k)).toList
    }).toList
  }

  def part1(lines: List[List[Char]]): Int = {
    val horizontals = lines
    val verticals = lines.transpose
    val rightDiagonals = diagonals(lines)
    val leftDiagonals = diagonals(lines.map(_.reverse))

    counts(horizontals) + counts(verticals) + counts(rightDiagonals) + counts(leftDiagonals)
  }

  def part2(lines: List[List[Char]]): Unit = {}
}
