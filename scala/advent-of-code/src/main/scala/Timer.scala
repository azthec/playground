// https://biercoff.com/easily-measuring-code-execution-time-in-scala/
object Timer {
  def time[R](block: => R): (Double, R) = {
    val t0 = System.nanoTime()
    val result = block // call-by-name
    val t1 = System.nanoTime()
    val dif = (t1 - t0) / 1e6
    (dif, result)
  }
  def lots[F](n: Int, f: => F): F = if (n <= 1) f else { f; lots(n - 1, f) }
}
