use crate::geometry::discmesh::{Cell, CellMesh, TetrahedralMesh, Tetrahedron};
use crate::geometry::polymesh::{PolyMesh, TriangleMesh};

pub trait Discretizer<T: PolyMesh, U: Cell, V: CellMesh<U>> {
    fn discretize(polymesh: &T) -> V;
}

pub struct TetrahedralDiscretizer {}

impl Discretizer<TriangleMesh, Tetrahedron, TetrahedralMesh> for TetrahedralDiscretizer {
    fn discretize(polymesh: &TriangleMesh) -> TetrahedralMesh {
        todo!()
    }
}
