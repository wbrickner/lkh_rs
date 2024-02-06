use std::{fs::OpenOptions, io::{BufReader, Read}, path::PathBuf, str::{FromStr, Lines}};
use crate::util::string::{string, PoolString};

pub struct TourData<R> { reader: R }

impl TourData<PoolString> {
  pub fn from_file(file: &PathBuf) -> std::io::Result<Self> {
    let reader = OpenOptions::new().read(true).open(file)?;
    let reader = {
      let mut s = string();
      BufReader::new(reader)
        .read_to_string(&mut s)?;
      s
    };
    Ok(Self {
      reader
    })
  }

  pub fn parse(&self) -> TourParse { TourParse::from(self) }
}

pub struct TourParse<'a>(Lines<'a>);

impl<'a> From<&'a TourData<PoolString>> for TourParse<'a> {
  fn from(data: &'a TourData<PoolString>) -> Self {
    Self(data.reader.lines())
  }
}

impl<'a> TourParse<'a> {
  fn parse<'b>(&'b mut self, id: &'b impl AsRef<str>) -> std::io::Result<&'b str> {
    while let Some(line) = self.0.next() {
      if line.starts_with(id.as_ref()) {
        if let Some(location) = line.find(':').or_else(|| line.find('=')) {
          return Ok(&line[location..].trim())
        } else {
          return Ok(&line.trim())
        }
      }
    }

    Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "could not find keyword"))    
  }

  fn parse_u32(n: &str, s: &str) -> std::io::Result<u32> {
    u32::from_str(s)
      .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, format!("{n} is not a number")))
  }

  pub fn dimension(&mut self) -> std::io::Result<u32> {
    self
      .parse(&"DIMENSION")
      .and_then(|s| Self::parse_u32("DIMENSION", s))
  }

  pub fn tour(&mut self) -> std::io::Result<Vec<u32>> {
    let mut buffer = vec![];

    self.parse(&"TOUR_SECTION")?;

    (&mut self.0)
      .map(|s| s.trim())
      .map_while(|s| {
        if s != "-1" { Some(s) } else { None } })
      .try_for_each(|entry| {
        let r = Self::parse_u32("TOUR_SECTION", entry)?;
        buffer.push(r - 1);
        std::io::Result::<()>::Ok(())
      })?;

    Ok(buffer)
  }
}
