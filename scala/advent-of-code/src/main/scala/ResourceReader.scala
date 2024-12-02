import scala.io.Source
import scala.util.{Using, Try}
import scala.util.Failure
import scala.util.Success

object ResourceReader {
  def load(filename: String): List[String] = {
    Using(getClass.getClassLoader.getResourceAsStream(filename)) { stream =>
      assert(stream != null, s"File $filename not found")
      val source = Source.fromInputStream(stream)
      source.getLines().toList
    } match {
      case Failure(exception) => throw exception
      case Success(value)     => value
    }
  }
}
