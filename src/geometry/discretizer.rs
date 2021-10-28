use crate::geometry::discmesh::CellMesh;
use crate::geometry::polymesh::{PolyMesh, TriangleMesh};

pub trait Discretizer<T: PolyMesh> {
    fn discretize(polymesh: &T) -> CellMesh;
}

pub struct TetrahedralDiscretizer {

}

impl Discretizer<TriangleMesh> for TetrahedralDiscretizer {
    fn discretize(polymesh: &TriangleMesh) -> CellMesh {
        todo!()
    }
}