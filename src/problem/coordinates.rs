use std::io::Write;

use crate::edges::EdgeData;

/// A `Coordinates` can only be made by a `ProblemFile`.
pub struct Coordinates<W: Write>(W);

impl<W: Write> Coordinates<W> {
  #[inline(always)] #[must_use] 
  pub(crate) const fn new(w: W) -> Self { Self(w) }

  pub fn begin_node_coordinates(
    mut self,
  ) -> std::io::Result<Self> { 
    writeln!(self.0, "NODE_COORD_SECTION")?; Ok(self)
  }
  
  pub fn write_coordinate<const N: usize>(
    mut self,
    index: usize,
    coordinate: &[f32; N]
  ) -> std::io::Result<Self> { 
    let w = &mut self.0;
    write!(w, "{} ", index + 1)?; // LKH is 1-indexed (not 0-indexed)

    coordinate
      .into_iter()
      .try_for_each(|v| write!(w, "{:.10e} ", v))?;
      // 1 5.49480e+02 2.93629e+01

    writeln!(w)?;
    Ok(self)
  }
  
  pub fn write_coordinates<const N: usize>(
    self,
    coordinates: impl Iterator<Item = [f32; N]>
  ) -> std::io::Result<Self> {
    coordinates
      .into_iter()
      .enumerate()
      .try_fold(self, |s, (j, c)| {
        s.write_coordinate(j, &c)
      })
      .map(|s| s)
  }

  pub fn edges(self) -> EdgeData<W> { EdgeData::new(self.0) }

  pub fn finish(self) -> W { self.0 }
}