object y24d4 {
  def execute(input: List[String] = ResourceReader.load("y24d4")): Unit = {
    val reports = parse(input)

    val (timer1, result1) = Timer.time(part1(reports))
    val (timer2, result2) = Timer.time(part2(reports))
    println(f"Elapsed time: $timer1 ms | Result: ${result1}")
    println(f"Elapsed time: $timer2 ms | Result: ${result2}")
  }

  def parse(lines: List[String]): List[List[Char]] = lines.map(_.toList)

  def part1(lines: List[List[Char]]): Int = {
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
      val tophalf: List[List[Char]] =
        (for (i <- 1 until lines.size) yield {
          (for (j <- 0 until lines.size if i + j < lines.size) yield {
            lines(j)(i + j)
          }).toList
        }).toList

      val middle = List((0 to lines.size - 1).map { i => lines(i)(i) }.toList)

      val bottomhalf: List[List[Char]] =
        (for (i <- 1 until lines.size) yield {
          (for (j <- 0 until lines.size if i + j < lines.size) yield {
            lines(i + j)(j)
          }).toList
        }).toList

      return tophalf ++ middle ++ bottomhalf
    }

    val horizontals = lines
    val verticals = lines.transpose
    val rights = diagonals(lines)
    val lefts = diagonals(lines.map(_.reverse))

    counts(horizontals) + counts(verticals) + counts(rights) + counts(lefts)
  }

  def part2(lines: List[List[Char]]): Int = {
    (for
      x <- 1 to lines.size - 2 // skip first and last row
      y <- 1 to lines.size - 2 // skip first and last col
      if lines(x)(y) == 'A' // look for the middle letter
    yield {
      def sam(tuple: (Char, Char, Char)) = tuple match {
        case Tuple3('M', 'A', 'S') => true
        case Tuple3('S', 'A', 'M') => true
        case _                     => false
      }

      val right = (lines(x - 1)(y - 1), lines(x)(y), lines(x + 1)(y + 1))
      val left = (lines(x + 1)(y - 1), lines(x)(y), lines(x - 1)(y + 1))

      sam(right) && sam(left)
    }).count(identity)
  }
}
