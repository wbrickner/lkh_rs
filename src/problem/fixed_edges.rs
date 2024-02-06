use std::io::Write;
use crate::Edge;

pub struct FixedEdges<W: Write>(W);

impl<W: Write> FixedEdges<W> {
  #[inline(always)] #[must_use] 
  pub(crate) const fn new(w: W) -> Self { Self(w) }

  /// edges we demand appear in the solution
  pub fn begin_fixed_edges(
    mut self
  ) -> std::io::Result<Self> {
    writeln!(&mut self.0, "FIXED_EDGES_SECTION").map(|_| self)
  }

  pub fn write_fixed_edges(
    mut self,
    edges: impl Iterator<Item = Edge>
  ) -> std::io::Result<Self> {
    edges
    .into_iter()
    .try_for_each(|c| {
      c
        .into_iter()
        .try_for_each(|v| write!(&mut self.0, "{} ", v))?;
  
      writeln!(&mut self.0)
    })
    .map(|_| self)
  }

  pub fn finish(self) -> W { self.0 }
}
