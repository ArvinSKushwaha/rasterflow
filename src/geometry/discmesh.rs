/*! This module contains classes to represent discretized meshes. The `CellMesh` struct represents
the volume of an object implementing `PolyMesh` comprised of 3-D volume elements (`Cell`). */
use nalgebra::{vector, Point, Unit};

use crate::{Float, Int, Point3, Uint, UnitVec3, Vec3};

/// The `Cell` trait is used to label structs as valid cells. The set of associated methods for
/// this trait are as follows:
pub trait Cell: PartialEq + Clone {}

/// The `CellMesh` traits represents a set of properties used to represent a volumetric mesh
/// comprised of cells implementing the `Cell` trait. This cell trait is given by the type
/// parameter `T`.
pub trait CellMesh<T: Cell> {
    fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = &'a T> + 'a>;
}

/// A `Tetrahedron` is a type that implements the `Cell` trait. It is a struct tuple containing a fixed-size array of 4 3D points.
#[derive(Clone)]
pub struct Tetrahedron([Point3; 4]);

/// The `TetrahedralMesh` struct represents the collection of connected vertices, edges, and facee comprising a set of tetrahedra.
pub struct TetrahedralMesh {
    vertices: Vec<Point3>,
    faces: Vec<[Uint; 3]>,
}

const PERM4: [[usize; 4]; 24] = [
    [0, 1, 2, 3],
    [0, 1, 3, 2],
    [0, 2, 1, 3],
    [0, 2, 3, 1],
    [0, 3, 1, 2],
    [0, 3, 2, 1],
    [1, 0, 2, 3],
    [1, 0, 3, 2],
    [1, 2, 0, 3],
    [1, 2, 3, 0],
    [1, 3, 0, 2],
    [1, 3, 2, 0],
    [2, 0, 1, 3],
    [2, 0, 3, 1],
    [2, 1, 0, 3],
    [2, 1, 3, 0],
    [2, 3, 0, 1],
    [2, 3, 1, 0],
    [3, 0, 1, 2],
    [3, 0, 2, 1],
    [3, 1, 0, 2],
    [3, 1, 2, 0],
    [3, 2, 0, 1],
    [3, 2, 1, 0],
];

impl PartialEq for Tetrahedron {
    fn eq(&self, other: &Self) -> bool {
        PERM4
            .iter()
            .any(|x| (0..4).all(|i| self.0[x[i]].eq(&other.0[i])))
    }
}
impl Cell for Tetrahedron {}

impl CellMesh<Tetrahedron> for TetrahedralMesh {
    fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = &'a Tetrahedron> + 'a> {
        todo!()
    }
}
