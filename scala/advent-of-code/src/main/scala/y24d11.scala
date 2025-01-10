import math._
import scala.collection.mutable

object y24d11 {
  def execute(input: Seq[String] = ResourceReader.load("y24d11")): Unit = {
    val parsed = parse(input)

    val (timer1, result1) = Timer.time(part1(parsed))
    val (timer2, result2) = Timer.time(part2(parsed))
    println(f"Elapsed time: $timer1 ms | Result: ${result1}")
    println(f"Elapsed time: $timer2 ms | Result: ${result2}")
  }

  def digits(number: Long): Int =
    if (number == 0) 1 else floor(log10(number)).toInt + 1

  def split(number: Long, digits: Int): LazyList[Long] = {
    val divisor = math.pow(10, digits).toLong
    val firstPart = floor(number / divisor).toLong
    val secondPart = number % divisor
    LazyList(firstPart, secondPart)
  }

  def iterate(sequence: Seq[Long], repeats: Int): Long = {
    val ruleCache = mutable.Map[(Long, Int), LazyList[Long]]()
    var total = 0

    def get(stone: Long, repeats: Int): LazyList[Long] = {
      if (repeats == 0) {
        val result = rules(stone)
        ruleCache.update((stone, repeats), result)
        result
      } else {
        ruleCache.getOrElseUpdate(
          (stone, repeats),
          get(stone, repeats - 1).flatMap(rules)
        )
      }
    }

    var lastTime = System.nanoTime()
    val res = sequence
      .flatMap(stone => {
        val time = System.nanoTime()
        if ((time - lastTime) / 1e6 > 1000) {
          println(f"stone: [$stone], cache: [${ruleCache.size}]")
          lastTime = time
        }
        get(stone, repeats - 1)
      })
      .size
    println(f"cache: [${ruleCache.size}]")
    res
  }

  def rules(stone: Long): LazyList[Long] = {
    val ndigits = digits(stone)
    stone match {
      case 0                     => LazyList(1)
      case _ if ndigits % 2 == 0 => split(stone, ndigits / 2)
      case _                     => LazyList(stone * 2024)
    }
  }

  def part1(stones: Seq[Long]): Long = iterate(stones, 25)

  def part2(stones: Seq[Long]): Long = iterate(stones, 40)

  def parse(lines: Seq[String]): Seq[Long] =
    lines.flatMap(_.split("\\s+").map(_.toLong));
}
