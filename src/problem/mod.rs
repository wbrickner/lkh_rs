use std::io::Write;
use self::header::Header;

pub mod header;
pub mod edges;
pub mod coordinates;
pub mod fixed_edges;

#[derive(Default)]
pub struct Problem<W = Vec<u8>>(W) where W: Write;

impl<W: Write> Problem<W> {
  pub fn with(w: W) -> Self { Self(w) }
  pub fn header(self) -> Header<W> { Header::new(self.0) }
}
