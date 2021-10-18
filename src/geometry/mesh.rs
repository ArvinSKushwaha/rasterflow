use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;


pub struct PolygonMesh {
    pub vertices: Vec<(f32, f32, f32)>,
    pub faces: Vec<HashSet<u32>>, // A vector of a set of indices representing a set of vertices.
    pub cells: Vec<HashSet<u32>> // A vector of a set of indices representing a set of faces.
}

impl PolygonMesh {
    /// Loads a `PolygonMesh` from the filename passed in.
    pub fn load_obj(filename: String) -> PolygonMesh {
        let mut polymesh = PolygonMesh {
            vertices: Vec::with_capacity(4),
            faces: Vec::with_capacity(4),
            cells: Vec::with_capacity(1),
        };

        // To implement file processing
        let file = match File::open(filename) {
            Ok(file) => file,
            Err(_) => return polymesh
        };

        // Assists with processing files using a buffer (to save us from the catastrophe that large
        // files can cause
        let mut bufread = BufReader::new(file);

        polymesh
    }
}