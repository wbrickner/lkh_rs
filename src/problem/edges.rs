use std::io::Write;
use crate::{fixed_edges::FixedEdges, util, Edge, Node};

pub struct EdgeData<W: Write>(W);

impl<W: Write> EdgeData<W> {
  #[inline(always)] #[must_use] 
  pub(crate) const fn new(w: W) -> Self { Self(w) }

  pub fn begin_adjacency(
    mut self,
  ) -> std::io::Result<Self> {
    writeln!(self.0, "EDGE_DATA_SECTION").map(|_| self)
  }
  
  pub fn begin_adjacency_edge<'a>(
    mut self,
    source: Node
  ) -> std::io::Result<Self> {
    write!(self.0, "{} ", source + 1).map(|_| self)
  }
  
  pub fn write_adjacency_entry(
    mut self,
    destination: Node
  ) -> std::io::Result<Self> {
    write!(self.0, "{} ", destination + 1).map(|_| self)
  }
  
  fn end_adjacency_edge(mut self) -> std::io::Result<Self> {
    util::end_seq(&mut self.0).map(|_| self)
  }
  
  /// the `node_edges` iterator needs to yield repeats of the same
  /// source node, each source node having one and only one block.
  pub fn write_adjacency_edges<'a>(
    mut self,
    mut node_edges: impl Iterator<Item = Edge>
  ) -> std::io::Result<Self> {
  
    let mut current = None;

    if let Some([start, _]) = node_edges.next() {
      self = self.begin_adjacency_edge(start)?;
      current = Some(start);
    }

    for [start, end] in node_edges {
      match current {
        Some(current) if current != start => {
          self = self
            .end_adjacency_edge()?
            .begin_adjacency_edge(start)?
        },
        _ => {}
      };
      current = Some(start);

      self = self.write_adjacency_entry(end)?;
    }

    self.end_adjacency_edge()
  }

  pub fn fixed_edges(self) -> FixedEdges<W> { FixedEdges::new(self.0) }

  pub fn finish(self) -> W { self.0 }
}
