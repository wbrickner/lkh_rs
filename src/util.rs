use std::io::Write;
use std::fmt::Display;

use static_init::dynamic;

pub fn write_sequence<C: Display, const N: usize>(
  w: &mut impl Write,
  coordinate: &[C; N]
) -> std::io::Result<()> {
  coordinate
    .into_iter()
    .try_for_each(|v| write!(w, "{} ", v))?;
  writeln!(w)
}

pub fn end_seq(w: &mut impl Write) -> std::io::Result<()> {
  write!(w, "-1")?;
  writeln!(w)
}

pub mod string {
  use super::*;
  #[dynamic] static STRING_POOL: object_pool::Pool<String> = object_pool::Pool::new(3, || String::with_capacity(4096));
  pub type PoolString = object_pool::Reusable::<'static, String>;
  pub fn string() -> PoolString {
    let mut s = STRING_POOL.pull(|| String::with_capacity(4096));
    s.clear();
    s
  }
  pub fn string_with_capacity(capacity: usize) -> PoolString {
    STRING_POOL.pull(|| String::with_capacity(capacity))
  }
}

pub mod float_buffer {
  use super::*;
  #[dynamic] static FLOAT_POOL: object_pool::Pool<Vec<f32>> = object_pool::Pool::new(1, || Vec::with_capacity(1024));

  pub type FloatBuffer = object_pool::Reusable::<'static, Vec<f32>>;
  pub fn float_buffer() -> FloatBuffer {
    let mut s = FLOAT_POOL.pull(|| Vec::with_capacity(1024));
    s.clear();
    s
  }

  pub fn string_with_capacity(capacity: usize) -> FloatBuffer {
    FLOAT_POOL.pull(|| Vec::with_capacity(capacity))
  }
}