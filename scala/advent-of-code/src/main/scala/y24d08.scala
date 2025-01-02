object y24d8 {
  def execute(input: Seq[String] = ResourceReader.load("y24d8")): Unit = {
    val parsed = parse(input)

    val (timer1, result1) = Timer.time(part1(parsed))
    val (timer2, result2) = Timer.time(part2(parsed))
    println(f"Elapsed time: $timer1 ms | Result: ${result1}")
    println(f"Elapsed time: $timer2 ms | Result: ${result2}")
  }

  case class Antenna(char: Char, x: Int, y: Int)

  def part1(lines: Seq[Seq[Char]]): Int = {
    val antennas = getAntennas(lines)
    val resonant = getResonantAntennas(antennas)
    val anti = antiNodes(resonant)

    def in(antenna: Antenna) =
      (0 <= antenna.x && antenna.x < lines.size && 0 <= antenna.y && antenna.y < lines.size)

    anti.distinct.filter(in(_)).size
  }

  def part2(lines: Seq[Seq[Char]]): Int = {
    val antennas = getAntennas(lines)
    val resonant = getResonantAntennas(antennas)
    val anti = antiNodes(resonant, 0, 100)

    def in(antenna: Antenna) =
      (0 <= antenna.x && antenna.x < lines.size && 0 <= antenna.y && antenna.y < lines.size)

    anti.distinct.filter(in(_)).size
  }

  def getAntennas(lines: Seq[Seq[Char]]) = for {
      x <- 0 to lines.size - 1
      y <- 0 to lines(x).size - 1
      char = lines(x)(y)
      if char != '.'
    } yield Antenna(char, x, y)

  def getResonantAntennas(antennas: Seq[Antenna]): Seq[(Antenna, Antenna)] =
    antennas
      .groupBy(_.char)
      .values
      .flatMap { case resonantAntennas: Seq[Antenna] =>
        resonantAntennas.combinations(2).collect { case Seq(left, right) =>
          (left, right)
        }
      }
      .toSeq

  def antiNodes(
      resonants: Seq[(Antenna, Antenna)],
      from: Int = 1,
      repeats: Int = 1
  ): Seq[Antenna] =
    resonants.foldLeft(Seq[Antenna]()) { case (fold, (antenna1, antenna2)) =>
      var res: Seq[Antenna] = (from to repeats).foldLeft(Seq[Antenna]()) {
        case (acc, r) =>
          acc ++ Seq(
            Antenna(
              '#',
              antenna1.x - r * (antenna2.x - antenna1.x),
              antenna1.y - r * (antenna2.y - antenna1.y)
            ),
            Antenna(
              '#',
              antenna2.x + r * (antenna2.x - antenna1.x),
              antenna2.y + r * (antenna2.y - antenna1.y)
            )
          )
      }

      fold ++ res
    }

  def parse(lines: Seq[String]): Seq[Seq[Char]] =
    lines.map(_.map(_.toChar).toSeq)

}
