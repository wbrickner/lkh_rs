#![allow(dead_code)]
use std::{path::PathBuf, process::Command};

pub type Node = u32;
pub type Edge = [Node; 2];
pub type SpatialCoordinate<const N: usize> = [f32; N];

mod problem; use memory_file::tmp_file;
use problem::{header::{EdgeFormat, EdgeWeightKind, ProblemKind}, *};

use crate::tour::*;
mod parameter;
pub mod memory_file;
pub mod util; 
pub mod tour;

fn solve_files(
  tour_file: &PathBuf,
  parameter_file: &PathBuf
) -> std::io::Result<Vec<Node>> {
  println!("Solving...");

  let output = 
    Command::new("lkh")
      .arg(parameter_file)
      .output()?;

  println!("Output: {:?}", output);

  TourData::from_file(tour_file)?
    .parse()
    .tour()
}

use std::io::Write;

pub fn solve_toolpath(
  endpoint_pairs: &[ [[f32; 2]; 2] ]
) -> std::io::Result<Vec<Node>> {
  let problem_file = {
    let p = 
      Problem::with(Vec::with_capacity(4096))
        .header()
          .problem_kind(ProblemKind::TSP)?
          .dimension(endpoint_pairs.len() as u32 * 2)?
          .edge_weight_kind(EdgeWeightKind::EUC_2D)?
          .edge_data_format(EdgeFormat::ADJ_LIST)?
        .coords()
          .begin_node_coordinates()?
          .write_coordinates(
            endpoint_pairs.iter().flat_map(|p| p).map(|[a, b]| [100. * a, 100. * b])
          )?
        .edges()
          .fixed_edges()
            .begin_fixed_edges()?
            .write_fixed_edges(
              (0..endpoint_pairs.len() as u32)
                .map(|i| i + 1)
                .map(|i| 2 * i)
                .map(|i| [i - 1, i])
            )?
        .finish();

      let t = tmp_file("tsp");
      std::fs::write(&t, p)?;
      t
  };

  let tour_file = tmp_file("tour");

  let parameter_file = {
    let mut m = Vec::with_capacity(4096);
    parameter::problem_file(&mut m, problem_file.to_str().unwrap())?;
    // writeln!(&mut m, "MOVE_TYPE = 5")?;
    // writeln!(&mut m, "PATCHING_C = 3")?;
    // writeln!(&mut m, "PATCHING_A = 2")?;
    // writeln!(&mut m, "RUNS = 10")?;
    writeln!(&mut m, "POPULATION_SIZE = 256")?;
    writeln!(&mut m, "TOUR_FILE = {}", tour_file.to_str().unwrap())?;

    m.flush().unwrap();

    let p = tmp_file("par");
    println!("parameter file path: {:?}", p.to_str());
    println!("parameter file contents: {}", String::from_utf8(m.clone()).unwrap());

    std::fs::write(&p, m).unwrap();
    p
  };

  solve_files(&tour_file, &parameter_file)
}

#[cfg(test)]
mod test {
  use std::io::Write;
  use rand::{thread_rng, Rng};
  use crate::{header::{EdgeFormat, EdgeWeightKind, ProblemKind}, memory_file::tmp_file, parameter, solve_toolpath, Problem};

  #[test]
  fn xx() {
    println!("{:#?}", solve_toolpath(
      &[
        [
          [0., 0.],
          [1., 0.],
        ],
        [
          [0., 1.],
          [1., 1.],
        ]
      ]
    ));
  }

  #[test]
  fn test_solve() {
    const DIM: u32 = 10;

    (|| {
      let problem_file = {
        let m = 
          Problem::with(Vec::with_capacity(4096))
            .header()
              .name("test")?
              .problem_kind(ProblemKind::TSP)?
              .comment("test comment")?
              .dimension(DIM)?
              .edge_weight_kind(EdgeWeightKind::EUC_2D)?
              .edge_data_format(EdgeFormat::ADJ_LIST)?
            .coords()
              .begin_node_coordinates()?
              .write_coordinates(
                (0..DIM).map(|_| thread_rng().gen::<[f32; 2]>()).map(|[a, b]| [100. * a, 100. * b])
              )?
            // .edges()
            //   .begin_adjacency()?
            //   .write_adjacency_edges(
            //     (0..DIM)
            //       .flat_map(|i| (0..DIM).filter(move |j| *j != i).map(move |j| [i, j]))
            //   )?
            .finish();

        let p = tmp_file("tsp");
        std::fs::write(&p, m)?;
        p
      };

      let tour_file = tmp_file("tour");

      let parameter_file = {
        let mut m = Vec::with_capacity(4096);
        parameter::problem_file(&mut m, problem_file.to_str().unwrap())?;
        // writeln!(&mut m, "MOVE_TYPE = 5")?;
        // writeln!(&mut m, "PATCHING_C = 3")?;
        // writeln!(&mut m, "PATCHING_A = 2")?;
        writeln!(&mut m, "RUNS = 2")?;
        writeln!(&mut m, "POPULATION_SIZE = 512")?;
        writeln!(&mut m, "TOUR_FILE = {}", tour_file.to_str().unwrap())?;

        m.flush().unwrap();

        let p = tmp_file("par");
        println!("parameter file path: {:?}", p.to_str());
        println!("parameter file contents: {}", String::from_utf8(m.clone()).unwrap());

        std::fs::write(&p, m).unwrap();
        p
      };

      let soln = super::solve_files(&tour_file, &parameter_file).unwrap();

      println!("soln: {:?}", soln);

      std::io::Result::Ok(())
    })().unwrap();
  }
}


// trait Or<A, B>: (AsRef::<A>::as_ref, AsRef::<B>::as_ref) { }
// trait Or {
//   type Current;
//   type Next: Or;
// }

// impl Or for () { type Current = (); type Next = (); }

// impl<A, B> Or for (A, B) {
//   type Current = A;
//   type Next = B;
// }

// impl<T, A, B> Or for T where T: AsRef<A> {
//   type Current = A;
//   type Next = B;
// }

// oh wow:
// Or::Next::Current

// struct Wrap<Z>(Z);
// trait Proves<P> { }
// struct Cons<A, Rest>(A, Rest);

// impl<A> Cons<A, ()> {
//   fn first(a: A) -> Cons<A, ()> { Cons(a, ()) }
//   fn add<B>(self, b: B) -> Cons<B, Self> { Cons(b, self) }
// }

// struct Holds<A, Proof>;

// impl<A, Rest: Or, Proof> Or for Cons<Holds<A, Proof>, Rest> where A: AsRef<Proof> {
//   type Current = A;
//   type Next = Rest;
// }

// impl<A, Rest: Or, Proof> Or for Cons<A, Rest> where Cons<Rest, Rest::Next>: Or<Current = Proof> {
//   type Current = A;
//   type Next = Rest;
// }

// // trait OrBound<A, B>: Or<Current = A, Next = Cons<B, >> { }

// fn test() {
//   let x = 
//     Cons::first(Holds(1, ProofA))
//       .add(Holds(2, ProofB));
// }

// Cons(Holds(1, ProofA), Cons(Holds(2, ProofB), ())

// struct Cons<A, Rest>(A, Rest);

// impl<T, P, Child> Or for T where T: Or<Next = Child>, Child: Proves<P> {
//   type Current = P;
//   type Next = Child;
// }

// impl<T, A, B> Or<(A, B)> for T where T: AsRef<A> { }
// impl<T, A, B> Or<(A, Wrap<B>)> for T where B: Or<B> { }

// struct Wrapper<A, B = ()>(A, Option<Struct<B>>);

// impl<A, B> Or<(A, B)> for Wrapper<A, B> where A: AsRef<B> { }
// impl<A, B> Or<(A, B)> for Wrapper<A, B> where B: Or { }
// impl<W, A> Or<(A, B)> for Wrapper<W> where W: AsRef<A> { }

// soln: you could generate a trait on the fly and
// impl it for the type, but that's a bit of a hack.  Like
// impl Or<Z> for AutogenStruct where AutogenStruct: AsRef<B> { }

// impl<A, B> Or for Or<A, B> where A: AsRef<B> { }
// impl<A, B> Or<A, B> for Wrap<(A, B)> where A: AsRef<B> { }
// impl<A, B> Or<A, B> for Wrap<(A, B)> where A: AsRef<B> { }

// struct ProofA;
// struct ProofB;

// fn my_function(
//   x: impl Or<>
// ) {
// }

