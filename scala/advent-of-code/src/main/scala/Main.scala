object Main {
  def main(args: Array[String]): Unit = {
    args match {
      case Array()         => options()
      case Array("--help") => options()
      case Array("y24d1")  => y24d1.execute()
      case Array("y24d2")  => y24d2.execute()
      case Array("y24d3")  => y24d3.execute()
      case Array("y24d4")  => y24d4.execute()
      case Array("y24d5")  => y24d5.execute()
      case Array("y24d8")  => y24d8.execute()
      case Array("y24d9")  => y24d9.execute()
      case Array("y24d10")  => y24d10.execute()
      case _               => options(true)
    }
  }

  def options(invalid: Boolean = false): Unit = {
    if (invalid)
      println("Unknown option provided")
    val usage: String = """
      |Usage:
      |  sbt "run [options] [solutions]"
      |
      |Solutions:
      |  y24d1    Advent of Code 2024 Day 1
      |  y24d2    Advent of Code 2024 Day 2
      |  y24d3    Advent of Code 2024 Day 3
      |  y24d4    Advent of Code 2024 Day 4
      |  y24d5    Advent of Code 2024 Day 5
      |  y24d8    Advent of Code 2024 Day 8
      |  y24d9    Advent of Code 2024 Day 9
      |  y24d10   Advent of Code 2024 Day 10
      |
      |Options:
      |  none
      |""".stripMargin
    println(usage)
  }
}
