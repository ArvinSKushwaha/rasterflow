/*! This module contains classes to represent discretized meshes. The `CellMesh` struct represents
the volume of an object implementing `PolyMesh` comprised of 3-D volume elements (`Cell`). */



struct Cell {

}

pub struct CellMesh {
    cells: Vec<Cell>,
}