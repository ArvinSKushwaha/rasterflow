use crate::geometry::discmesh::{Cell, CellMesh, TetrahedralMesh, Tetrahedron};
use crate::geometry::polymesh::{PolyMesh, TriangleMesh};

// Define a set of helper functions (but split them into modules

/// The `delaunay` module provides helper functions
pub(in crate::geometry) mod delaunay {}

pub trait DiscretizerConfig {}

pub trait Discretizer<T: PolyMesh, U: Cell, V: CellMesh<U>, W: DiscretizerConfig> {
    fn discretize(polymesh: &T, config: &W) -> V;
}

pub struct TetrahedralDiscretizer {}

pub struct TetrahedralDiscretizerConfig {
    pub threshold_angle: f32,
}

impl DiscretizerConfig for TetrahedralDiscretizerConfig {}

impl Discretizer<TriangleMesh, Tetrahedron, TetrahedralMesh, TetrahedralDiscretizerConfig>
    for TetrahedralDiscretizer
{
    fn discretize(
        polymesh: &TriangleMesh,
        config: &TetrahedralDiscretizerConfig,
    ) -> TetrahedralMesh {
        todo!()
    }
}
