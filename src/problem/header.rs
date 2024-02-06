use std::io::Write;
use crate::coordinates::Coordinates;

#[derive(Debug)]
#[non_exhaustive]
pub enum ProblemKind {
  /// Data for a symmetric traveling salesman problem
  TSP,
  /// Data for an asymmetric traveling salesman problem
  ATSP,
  /// Data for a sequential ordering problem
  SOP,
  /// Hamiltonian cycle problem data
  HCP,
  /// Capacitated vehicle routing problem data
  CVRP,
  /// A collection of tours
  TOUR,
}

pub struct Header<W: Write>(W);

impl<W: Write> Header<W> {
  #[inline(always)] #[must_use] 
  pub(crate) const fn new(w: W) -> Self { Self(w) }

  /// Identifies the data file.
  pub fn name(
    mut self,
    name: &str
  ) -> std::io::Result<Self> {
    writeln!(&mut self.0, "NAME: {}", name)?;
    Ok(self)
  }

  /// Specifies the type of the problem
  pub fn problem_kind(
    mut self,
    kind: ProblemKind
  ) -> std::io::Result<Self> {
    writeln!(&mut self.0, "TYPE: {:?}", kind)?;
    Ok(self)
  }

  /// Additional comments (usually the name of the contributor or creator of the problem instance is given here).
  pub fn comment(
    mut self,
    comment: &str
  ) -> std::io::Result<Self> {
    writeln!(&mut self.0, "COMMENT: {}", comment)?;
    Ok(self)
  }

  /// For a TSP or ATSP, the dimension is the number of its nodes. For a CVRP, it is the total
  /// number of nodes and depots. For a TOUR file it is the dimension of the corresponding
  /// problem.
  pub fn dimension(
    mut self,
    dimension: u32
  ) -> std::io::Result<Self> {
    writeln!(&mut self.0, "DIMENSION: {}", dimension)?;
    Ok(self)
  }

  /// Specifies the truck capacity in a CVRP.
  pub fn capacity(
    mut self,
    capacity: u32
  ) -> std::io::Result<Self> {
    writeln!(&mut self.0, "CAPACITY: {}", capacity)?;
    Ok(self)
  }

  /// Specifies how the edge weights (or distances) are given. 
  /// The values are 
  pub fn edge_weight_kind(
    mut self,
    kind: EdgeWeightKind
  ) -> std::io::Result<Self> {
    writeln!(&mut self.0, "EDGE_WEIGHT_TYPE: {:?}", kind)?;
    Ok(self)
  }

  pub fn edge_weight_format(
    mut self,
    format: EdgeWeightFormat
  ) -> std::io::Result<Self> {
    writeln!(&mut self.0, "EDGE_WEIGHT_FORMAT: {:?}", format)?;
    Ok(self)
  }
  
  pub fn edge_data_format(
    mut self,
    format: EdgeFormat
  ) -> std::io::Result<Self> {
    writeln!(&mut self.0, "EDGE_DATA_FORMAT: {:?}", format)?;
    Ok(self)
  }
  
  pub fn node_coord_kind(
    mut self,
    kind: CoordinateKind
  ) -> std::io::Result<Self> {
    writeln!(&mut self.0, "NODE_COORD_TYPE: {:?}", kind)?;
    Ok(self)
  }
  
  pub fn display_data_kind(
    mut self,
    kind: DisplayDataKind
  ) -> std::io::Result<Self> {
    writeln!(&mut self.0, "DISPLAY_DATA_TYPE: {:?}", kind)?;
    Ok(self)
  }

  pub fn coords(
    self,
  ) -> Coordinates<W> {
    Coordinates::new(self.0)
  }

  pub fn finish(self) -> W { self.0 }
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
/// Specifies how the edge weights (or distances) are given. The values are
/// 
/// - `EXPLICIT`: Weights are listed explicitly in the corresponding section
/// - `EUC_2D`: Weights are Euclidean distances in 2-D
/// - `EUC_3D`: Weights are Euclidean distances in 3-D
/// - `MAX_2D`: 2D Weights are maximum distances in 2-D
/// - `MAX_3D`: 3D Weights are maximum distances in 3-D
/// - `MAN_2D`: 2D Weights are Manhattan distances in 2-D
/// - `MAN_3D`: 3D Weights are Manhattan distances in 3-D
/// - `CEIL`: 2D Weights are Euclidean distances in 2-D rounded up
/// - `GEO`: Weights are geographical distances
/// - `ATT`: Special distance function for problems att48 and att532
/// - `XRAY1`: Special distance function for crystallography problems (Version 1)
/// - `XRAY2`: Special distance function for crystallography problems (Version 2)
/// - `SPECIAL`: There is a special distance function documented elsewhere
pub enum EdgeWeightKind {
  /// Weights are listed explicitly in the corresponding section
  EXPLICIT,
  /// Weights are Euclidean distances in 2-D
  EUC_2D,
  /// Weights are Euclidean distances in 3-D
  EUC_3D,
  /// 2D Weights are maximum distances in 2-D
  MAX_2D,
  /// 3D Weights are maximum distances in 3-D
  MAX_3D,
  /// 2D Weights are Manhattan distances in 2-D
  MAN_2D,
  /// 3D Weights are Manhattan distances in 3-D
  MAN_3D,
  /// 2D Weights are Euclidean distances in 2-D rounded up
  CEIL,
  /// Weights are geographical distances
  GEO,
  /// Special distance function for problems att48 and att532
  ATT,
  /// Special distance function for crystallography problems (Version 1)
  XRAY1,
  /// Special distance function for crystallography problems (Version 2)
  XRAY2,
  /// There is a special distance function documented elsewhere
  SPECIAL,
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
pub enum EdgeWeightFormat {
  /// Weights are given by a function (see above)
  FUNCTION,
  /// Weights are given by a full matrix
  FULL_MATRIX,
  /// Upper triangular matrix (row-wise without diagonal entries)
  UPPER_ROW,
  /// Lower triangular matrix (row-wise without diagonal entries)
  LOWER_ROW,
  /// Upper triangular matrix (row-wise including diagonal entries)
  UPPER_DIAG_ROW,
  /// Lower triangular matrix (row-wise including diagonal entries)
  LOWER_DIAG_ROW,
  /// Upper triangular matrix (column-wise without diagonal entries)
  UPPER_COL,
  /// Lower triangular matrix (column-wise without diagonal entries)
  LOWER_COL,
  /// Upper triangular matrix (column-wise including diagonal entries)
  UPPER_DIAG_COL,
  /// Lower triangular matrix (column-wise including diagonal entries
  LOWER_DIAG_COL,
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
pub enum EdgeFormat {
  /// The graph is given by an edge list
  EDGE_LIST,
  /// The graph is given as an adjacency list
  ADJ_LIST,
}

impl Default for EdgeFormat {
  fn default() -> Self { Self::ADJ_LIST }
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
pub enum CoordinateKind {
  /// Nodes are specified by coordinates in 2-D
  TWOD_COORDS,
  /// Nodes are specified by coordinates in 3-D
  THREED_COORDS,
  /// The nodes do not have associated spatial coordinates
  NO_COORDS,
}

impl Default for CoordinateKind {
  fn default() -> Self { Self::NO_COORDS }
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
pub enum DisplayDataKind {
  /// Display is generated from the node coordinates
  COORD_DISPLAY,
  /// Explicit coordinates in 2-D are given
  TWOD_DISPLAY,
  /// No graphical display is possible
  NO_DISPLAY,
}

impl Default for DisplayDataKind {
  fn default() -> Self { Self::COORD_DISPLAY }
}
