use std::{ffi::CStr, fs::OpenOptions, io::{Error, Write}, path::PathBuf};
use memmap::MmapMut;
use zeroize::Zeroize;

pub struct MemoryFile {
  pub path: PathBuf,
  pub cursor: usize,
  pub writer: MmapMut
}

impl Drop for MemoryFile {
  fn drop(&mut self) {
    if let Err(e) = self.wipe() { eprintln!("{e}") }
    if let Err(e) = std::fs::remove_file(&self.path) { eprintln!("{e}") }
  }
}

impl MemoryFile {
  pub fn default() -> std::io::Result<Self> {
    make_writer("mem", None)
      .map(|(path, writer)| Self { path, writer, cursor: 0 })
  }

  pub fn path(&self) -> &PathBuf { &self.path }

  pub fn wipe(&mut self) -> std::io::Result<()> {
    self.cursor.zeroize(); 
    self.writer.zeroize(); 
    self.flush()
      .map_err(|e| {
        Error::new(
          e.kind(), 
          format!(
"Security error. MemoryFile::wipe failed. In-memory contents zeroized, but zeroizing write to disk failed. Data may persist which exposes the contents of the TSP: {files:?}
Original error: {e}",
            files = std::env::temp_dir()
          )
        )
      })
  }

  pub fn resize(&mut self, new_size: u64) -> std::io::Result<()> {
    let (new_path, new_writer) = {
      let (p, mut w) = make_writer(
        self.path.extension().unwrap().to_str().unwrap(),
        Some(new_size)
      )?;
      self.writer.into_iter().zip(w.iter_mut()).for_each(|(s, d)| *d = *s);
      println!("Growing to: {}", w.len());
      println!("Contents: {:?}", CStr::from_bytes_until_nul(w.as_ref()).unwrap());
      (p, w)
    };

    self.wipe()?;
    self.writer = new_writer;

    std::fs::remove_file(&self.path)?;

    self.path = new_path;
    self.writer.flush()
  }

  pub fn grow(&mut self) -> std::io::Result<()> {
    self.resize(self.writer.len() as u64 * 2)
  }
}

impl Write for MemoryFile {
  fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
    if self.cursor + buf.len() > self.writer.len() {
      self.grow()?;
    }

    let r = (&mut self.writer.as_mut()[self.cursor..]).write(buf);
    if let Ok(n) = r {
      self.cursor += n;
    }
    r
  }

  fn flush(&mut self) -> std::io::Result<()> {
    self.writer.flush()
  }
}

use rand::{distributions::Alphanumeric, Rng};

pub fn tmp_file(extension: &str) -> PathBuf {
  std::env::temp_dir().join(
    &format!(
      "lkh_{}.tmp.{}", 
      {
        let mut rng = rand::thread_rng();
        std::iter::repeat(())
          .map(|()| rng.sample(Alphanumeric))
          .map(char::from)
          .take(24)
          .collect::<String>()
      },
      {
        extension
      }
    )
  )
}

pub fn make_writer(extension: &str, capacity: Option<u64>) -> std::io::Result<(PathBuf, MmapMut)> {
  let capacity = capacity.unwrap_or(4096);

  let path = tmp_file(extension);
  let file = OpenOptions::new()
    .read(true)
    .write(true)
    .create(true)
    .open(&path)?;

  file.set_len(capacity)?;

  let mut mm = unsafe { MmapMut::map_mut(&file) }?;
  assert!(mm.len() == capacity as usize, "mmap capacity mismatch");
  mm.fill(0);

  Ok((path, mm))
}
