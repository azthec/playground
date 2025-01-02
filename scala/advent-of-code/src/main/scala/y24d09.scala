import scala.collection.mutable.Stack
import scala.util.control.Breaks._
import scala.collection.mutable.ListBuffer
object y24d9 {
  def execute(input: Seq[String] = ResourceReader.load("y24d9")): Unit = {
    val parsed = parse(input)

    val (timer1, result1) = Timer.time(part1(parsed))
    // val (timer2, result2) = Timer.time(part2(parsed))
    println(f"Elapsed time: $timer1 ms | Result: ${result1}")
    // println(f"Elapsed time: $timer2 ms | Result: ${result2}")
  }

  enum Block {
    case File(id: Int, size: Int)
    case Free(size: Int)
  }

  def size(filesystem: Seq[Block]): Long = filesystem.collect {
    case Block.File(id, size) => size
    case Block.Free(size)     => size
  }.sum

  def checksum(filesystem: Seq[Block]): Long = {
    var index: Long = 0L
    var total: Long = 0L
    for (element <- filesystem) {
      element match {
        case Block.File(id, size) =>
          total += id * (index until index + size).sum.toLong
          index += size
        case Block.Free(size) =>
          index += size
      }
    }
    total
  }

  def part1(filesystem: Seq[Block]): Long = {
    var defragmented = Seq[Block]()
    var leftIndex = 0
    var rightIndex = size(filesystem)
    val listBuffer = filesystem.to(ListBuffer)
    var leftStack = filesystem.to(Stack)
    var rightStack = filesystem.reverse.to(Stack)

    breakable {
      while (leftIndex <= rightIndex) { // TODO check this condition

        var leftBlock = leftStack.pop()
        var rightBlock = rightStack.pop()

        (leftBlock, rightBlock) match {
          case (left @ Block.Free(_), right @ Block.File(_, _)) =>
            val remaining = left.size - right.size
            if (remaining > 0) { // more free space than needed
              val newLeft = Block.Free(remaining)
              leftStack.push(newLeft)
              defragmented = defragmented.appended(right)
              leftIndex += right.size
              rightIndex -= right.size
            } else if (remaining < 0) { // we don't have enough free space
              val leftOver = -remaining
              val newRight = Block.File(right.id, leftOver)
              rightStack.push(newRight)
              defragmented = defragmented.appended(
                Block.File(right.id, right.size - leftOver)
              )
              leftIndex += right.size - leftOver
              rightIndex -= leftOver
            } else {
              defragmented = defragmented.appended(right)
              leftIndex += right.size
              rightIndex -= right.size
            }
          case (left @ Block.Free(_), right @ Block.Free(_)) =>
            // don't skip free space on the left, skip free space on the right
            leftStack.push(left)
            rightIndex -= right.size
          case (left @ Block.File(_, _), right @ Block.File(_, _)) =>
            // append left file, don't skip file on the right
            defragmented = defragmented.appended(left)
            rightStack.push(right)
            leftIndex += left.size
          case (left @ Block.File(_, _), right @ Block.Free(_)) =>
            // append left file, skip free space on the right
            defragmented = defragmented.appended(left)
            leftIndex += left.size
            rightIndex -= right.size
        }
      }
    }

    checksum(defragmented)
  }

  def part2(lines: Seq[Int]): Long = ???

  def parse(lines: Seq[String]): Seq[Block] = {
    lines.flatMap(_.map(_.asDigit).zipWithIndex.map { case (size, index) =>
      if (index % 2 == 0)
        Block.File(index / 2, size)
      else
        Block.Free(size)
    })
  }
}
