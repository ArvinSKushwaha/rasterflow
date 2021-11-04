/*! This module contains classes to represent discretized meshes. The `CellMesh` struct represents
the volume of an object implementing `PolyMesh` comprised of 3-D volume elements (`Cell`). */
use nalgebra::{vector, Unit};

use crate::{Float, Int, Point3, Uint, UnitVec3, Vec3};

pub trait Cell {}

pub trait CellMesh<T: Cell> {}

pub struct Tetrahedron {}

pub struct TetrahedralMesh {
    cells: Vec<Tetrahedron>, // Each tetrahedron requires 4 vertices.
}

impl Cell for Tetrahedron {}

impl CellMesh<Tetrahedron> for TetrahedralMesh {}
