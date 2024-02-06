use std::io::Write;

pub fn problem_file(
  w: &mut impl Write,
  path: &str
) -> std::io::Result<()> {
  writeln!(w, "PROBLEM_FILE = {path}")
}
