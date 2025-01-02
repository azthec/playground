import scala.annotation.tailrec
object y24d10 {
  def execute(input: Seq[String] = ResourceReader.load("y24d10")): Unit = {
    val parsed = parse(input)

    val (timer1, result1) = Timer.time(part1(parsed))
    val (timer2, result2) = Timer.time(part2(parsed))
    println(f"Elapsed time: $timer1 ms | Result: ${result1}")
    println(f"Elapsed time: $timer2 ms | Result: ${result2}")
  }

  enum Consts(val value: Int) {
    case Start extends Consts(0)
    case End extends Consts(9)
  }

  val directions = Seq((-1, 0), (1, 0), (0, -1), (0, 1))

  def trail(matrix: Seq[Seq[Int]], start: (Int, Int)) = {
    def iter(x: Int, y: Int): Seq[(Int, Int)] = {
      safeMatrix(matrix, x, y) match {
        case None => return Seq() // out of bounds
        case Some(cur) => {
          // if we reached the end of a path
          if (cur == Consts.End.value)
            return Seq((x, y))
          // valid directions we can move to that respect the monotonically increasing rule
          val validDirections: Seq[(Int, Int)] = directions.filter {
            case (dx, dy) =>
              safeMatrix(matrix, x + dx, y + dy) match {
                case Some(value) => if (value == cur + 1) true else false
                case _           => false
              }
          }
          // recurse down the valid directions
          validDirections.flatMap { case (dx, dy) => iter(x + dx, y + dy) }
        }
      }
    }
    iter(start._1, start._2)
  }

  // get all starting positions
  def trailheads(map: Seq[Seq[Int]]): Seq[(Int, Int)] =
    for {
      (row, rowIndex) <- map.zipWithIndex
      (value, columnIndex) <- row.zipWithIndex
      if value == Consts.Start.value
    } yield (rowIndex, columnIndex)

  def part1(map: Seq[Seq[Int]]): Int = {
    trailheads(map).map { start => trail(map, start).distinct.length }.sum
  }

  def part2(map: Seq[Seq[Int]]): Int = {
    trailheads(map).map { start => trail(map, start).length }.sum
  }

  def parse(lines: Seq[String]): Seq[Seq[Int]] = lines.map(_.map(_.asDigit));

  def safeMatrix[T](matrix: Seq[Seq[T]], row: Int, col: Int): Option[T] =
    if (row >= 0 && row < matrix.length && col >= 0 && col < matrix(row).length)
      Some(matrix(row)(col))
    else None
}
