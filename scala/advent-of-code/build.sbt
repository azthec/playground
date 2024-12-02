val scala3Version = "3.5.2"

lazy val root = project
  .in(file("."))
  .settings(
    name         := "advent-of-code",
    version      := "0.1.0-SNAPSHOT",
    scalaVersion := scala3Version,
    scalacOptions ++= Seq("-no-indent"),
    libraryDependencies += "org.scala-lang" %% "scala3-library" % scalaVersion.value,
    libraryDependencies += "org.scalameta"  %% "munit"          % "1.0.2" % Test,
  )
