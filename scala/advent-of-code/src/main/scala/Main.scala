object Main {
  def main(args: Array[String]): Unit = {
    args match {
      case Array()         => options()
      case Array("--help") => options()
      case Array("y24d1")  => y24d1.execute()
      case Array("y24d2")  => y24d2.execute()
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
      |
      |Options:
      |  none
      |""".stripMargin
    println(usage)
  }
}
