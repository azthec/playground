object y24d5 {
  def execute(input: Seq[String] = ResourceReader.load("y24d5")): Unit = {
    val (rules, updates) = parse(input)

    val (timer1, result1) = Timer.time(part1(rules, updates))
    val (timer2, result2) = Timer.time(part2(rules, updates))
    println(f"Elapsed time: $timer1 ms | Result: ${result1}")
    println(f"Elapsed time: $timer2 ms | Result: ${result2}")
  }

  def isSorted(rules: Map[Int, Seq[Int]], pages: Seq[Int]): Boolean = {
    // this does force an unnecessary sort but it's easy to read :D
    sort(rules, pages) == pages
  }

  def sort(rules: Map[Int, Seq[Int]], pages: Seq[Int]): Seq[Int] =
    // b must show after a
    // if there is a rule of the form a | b
    pages.sortWith { (a, b) => rules.get(a).exists(_.contains(b)) }

  def middle(seq: Seq[Int]) = {
    assert(seq.size % 2 != 0, "Even sized lists have no middle")
    if (seq.size > 0) seq(seq.size / 2) else 0
  }

  def part1(rules: Map[Int, Seq[Int]], updates: Seq[Seq[Int]]): Int =
    updates.filter(isSorted(rules, _)).map(middle).sum

  def part2(rules: Map[Int, Seq[Int]], updates: Seq[Seq[Int]]): Int =
    updates.filter(!isSorted(rules, _)).map(sort(rules, _)).map(middle).sum

  def parse(lines: Seq[String]): (Map[Int, Seq[Int]], Seq[Seq[Int]]) = {
    lines.foldLeft((Map.empty[Int, Seq[Int]], Seq.empty[Seq[Int]])) {
      case ((map, list), page) =>
        page.split("[|,]") match {
          case Array(a, b) => // XX|YY
            (
              map.updated(
                a.toInt,
                map.getOrElse(a.toInt, Seq.empty) :+ b.toInt
              ),
              list
            )
          case arr: Array[String] if arr.length > 2 => // XX,YY,ZZ...
            (map, list :+ arr.map(_.toInt).toSeq)
          case Array("") =>
            (map, list)
          case _ =>
            throw new IllegalStateException("Invalid input format")
        }
    }
  }
}
