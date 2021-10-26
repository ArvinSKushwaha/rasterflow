use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Write};

use crate::{Float, Int, Point3, Uint, UnitVec3};
use nalgebra::Unit;
use regex::Regex;

/// `PolygonMesh` describes the input geometries pre-discretization for simulations.
pub struct PolygonMesh {
    vertices: Vec<Point3>,
    faces: Vec<Vec<usize>>,
    // A vector of a vector of indices representing a set of vertices.
    face_normals: Vec<UnitVec3>, // A vector of UnitVector3s
}

/// An enum containing error messages for PolygonMesh
#[derive(Eq, PartialEq)]
pub enum MeshError {
    IOError(&'static str),
    FormatError(&'static str),
    IndexingError(&'static str),
}

/**
Calculates the normals of a face. Assumes the points referenced by the face are counter-clockwise
and co-planar. This method takes the cross-product of `face[1] - face[0]` and `face[2] - face[0]`

- Vertex referenced in face could not be found: `MeshError::IndexingError("Could not load vertex.")`

Parameters:
- `&PolygonMesh` - The mesh containing the vertices to be referenced.
- `&[usize]` - The slice containing the indices of the vertices comprising this face.

Returns:
- `Result<UnitVec3, MeshError>` - Returns `Ok(UnitVec3)` if the method succeeds, else
`MeshError::IndexingError` if the method cannot retrieve the necessary vertices.
 */
pub fn get_face_normal(polymesh: &PolygonMesh, face: &[usize]) -> Result<UnitVec3, MeshError> {
    let point_origin = match polymesh.vertices.get(face[0]) {
        None => return Err(MeshError::IndexingError("Could not load vertex.")),
        Some(vertex) => vertex,
    };

    let first_point = match polymesh.vertices.get(face[1]) {
        None => return Err(MeshError::IndexingError("Could not load vertex.")),
        Some(vertex) => vertex,
    };

    let second_point = match polymesh.vertices.get(face[2]) {
        None => return Err(MeshError::IndexingError("Could not load vertex.")),
        Some(vertex) => vertex,
    };

    Ok(Unit::new_normalize(
        (first_point - point_origin).cross(&(second_point - point_origin)),
    ))
}

/**
A helper method to process strings from OBJ files into vertices.
This method may return `Some(MeshError)` if:
- Float cannot be processed: `MeshError::FormatError("Failed to parse float.")`
- Substrings could not generate: `MeshError::FormatError("Unable to process string.")`

Parameters:
- `polymesh: &mut PolygonMesh` - Reference to `PolygonMesh` object ot add vertices to.
- `vertex_string: &str` - String slice to process.

Returns:
- `Option<MeshReadError>` - If a failure occurred within the method. (Returns `None` if method
succeeded)
 */
fn process_obj_vertices(polymesh: &mut PolygonMesh, vertex_string: &str) -> Option<MeshError> {
    let mut point_strings = vertex_string.split_ascii_whitespace();

    let mut vertex_array: [Float; 3] = [0., 0., 0.];
    for i in &mut vertex_array {
        if let Some(numeric_string) = point_strings.next() {
            *i = match numeric_string.parse() {
                Ok(f) => f,
                Err(_) => {
                    (match numeric_string.parse::<Int>() {
                        Ok(f) => f,
                        Err(_) => {
                            return Some(MeshError::FormatError("Failed to parse float."));
                        }
                    }) as Float
                }
            };
        } else {
            return Some(MeshError::FormatError("Unable to process string."));
        }
    }
    polymesh.add_vertex(Point3::from(vertex_array));

    None
}

/**
A helper method to process strings from OBJ files into faces.
This method may return `Some(MeshError)` if:
- Integers cannot be processed: `MeshError::FormatError("Failed to parse integer.")`
- Substrings could not generate: `MeshError::FormatError("Failed to retrieve substring of face
element.")`
- Face has less than 3 vertices: `MeshError::FormatError("Face does not have enough vertices.")`
- Vertex referenced in face out of range: `MeshError::IndexingError("Vertex not contained in mesh")`
- Vertex referenced in face could not be found: `MeshError::IndexingError("Could not load vertex.")`

Parameters:
- `polymesh: &mut PolygonMesh` - Reference to `PolygonMesh` object ot add faces (and normals) to.
- `face_string: &str` - String slice to process.

Returns:
- `Option<MeshReadError>` - If a failure occurred within the method. (Returns `None` if method
succeeded)
 */
fn process_obj_faces(polymesh: &mut PolygonMesh, face_string: &str) -> Option<MeshError> {
    let face_strings = face_string.split_ascii_whitespace();

    let mut face: Vec<usize> = Vec::new();
    for i in face_strings {
        if let Some(index) = i.split('/').next() {
            let vertex = match index.parse::<Uint>() {
                Ok(i) => i,
                Err(_) => {
                    return Some(MeshError::FormatError("Failed to parse integer."));
                }
            } as usize
                - 1;
            if vertex < polymesh.get_vertex_count() {
                face.push(vertex);
            } else {
                return Some(MeshError::IndexingError("Vertex not contained in mesh."));
            }
        } else {
            return Some(MeshError::FormatError(
                "Failed to retrieve\
                                                        substring of face element.",
            ));
        }
    }

    if face.len() < 3 {
        return Some(MeshError::FormatError(
            "Face does not have enough verticies.",
        ));
    }

    match polymesh.add_face(face, None) {
        Ok(_) => {}
        Err(e) => return Some(e),
    }

    None
}

impl PolygonMesh {
    /**
    Loads a `PolygonMesh` from the filename passed in.

    Parameters:
    - `filename: &str` - A string containing the file path to load.

    Returns:
    - `Result<Box<PolygonMesh>, MeshError>` - Returns the `Box<PolygonMesh>` if the loading
    succeeded, otherwise a `MeshError` of some form, depending on the error.
     */
    pub fn load_obj(filename: &str) -> Result<Box<PolygonMesh>, MeshError> {
        let mut polymesh = PolygonMesh {
            vertices: Vec::with_capacity(4),
            faces: Vec::with_capacity(4),
            face_normals: Vec::with_capacity(4),
        };

        // To implement file processing
        let file: File;
        match File::open(filename) {
            Ok(f) => file = f,
            Err(e) => {
                return match e.kind() {
                    ErrorKind::NotFound => Err(MeshError::IOError("File not found.")),
                    ErrorKind::PermissionDenied => {
                        Err(MeshError::IOError("Insufficient permissions."))
                    }
                    _ => Err(MeshError::IOError("File failed to open.")),
                }
            }
        }

        // Assists with processing files using a buffer (to save us from the catastrophe that large
        // files can cause)
        let mut bufread = BufReader::new(file);
        let mut buffer_string = String::new();

        while match bufread.read_line(&mut buffer_string) {
            Ok(t) => t != 0,
            Err(_) => {
                return Err(MeshError::IOError("Could not read next line."));
            }
        } {
            buffer_string = buffer_string.trim().to_string();
            if buffer_string.starts_with("v ") {
                if let Some(error) =
                    process_obj_vertices(&mut polymesh, buffer_string.trim_start_matches("v "))
                {
                    return Err(error);
                }
                buffer_string = String::new();
                continue;
            } else if buffer_string.starts_with("f ") {
                if let Some(error) =
                    process_obj_faces(&mut polymesh, buffer_string.trim_start_matches("f "))
                {
                    return Err(error);
                }
                buffer_string = String::new();
                continue;
            } else if Regex::new("^(\\#|v(t|n|p)|g|o|s|usemtl|mtllib|l)")
                .unwrap()
                .is_match(buffer_string.as_str())
                || buffer_string.is_empty()
            {
                buffer_string = String::new();
                continue;
            } else {
                return Err(MeshError::FormatError("Invalid file line."));
            }
        }

        Ok(Box::new(polymesh))
    }

    /**
    Writes a `PolygonMesh` to the filename passed in.

    Parameters:
    - `filename: &str` - A string containing the filename to save them mesh to.

    Returns:
    - `Result<usize, Error>` - Returns the number of bytes written if file-writing is successful
    otherwise returns an `std::io::Error`, given by the methods called in this method.
     */
    pub fn write_obj(&self, filename: &str) -> Result<usize, Error> {
        let mut file = File::create(filename)?;
        let mut bytes: usize = 0;

        for vertex in &self.vertices {
            let string = format!("v {} {} {}", vertex.x, vertex.y, vertex.z);
            writeln!(file, "{}", string)?;
            bytes += string.len() + 1;
        }

        for face in &self.faces {
            let string: Vec<String> = face.iter().map(|f| (f + 1).to_string()).collect();
            let string = format!("f {}", string.join(" "));
            writeln!(file, "{}", string)?;
            bytes += string.len() + 1;
        }

        Ok(bytes)
    }

    /**
    Retrieves the number of vertices in the mesh.

    Returns:
    - `usize` - The number of the vertices.
     */
    #[inline(always)]
    pub fn get_vertex_count(&self) -> usize {
        self.vertices.len()
    }

    /**
    Retrieves the number of faces (and face normals) in the mesh.

    Returns:
    - `usize` - The number of faces (and face normals).
     */
    #[inline(always)]
    pub fn get_face_count(&self) -> usize {
        self.faces.len()
    }

    /**
    Gets a vertex at a given index.

    Parameters:
    - `idx: usize` - The index of the vertex to retrieve.

    Returns:
    - `Result<&Point3, MeshError>` - Returns `&Point3` if the indexing succeeds, else
    `MeshError::IndexingError`.
     */
    pub fn get_vertex(&self, idx: usize) -> Result<&Point3, MeshError> {
        self.vertices
            .get(idx)
            .ok_or(MeshError::IndexingError("Indexing failed."))
    }

    /**
    Gets a face at a given index.

    Parameters:
    - `idx: usize` - The index of the face to retrieve.

    Returns:
    - `Result<&Vec<usize>, MeshError>` - Returns `&Vec<usize>` if the indexing succeeds, else
    `MeshError::IndexingError`.
     */
    pub fn get_face(&self, idx: usize) -> Result<&Vec<usize>, MeshError> {
        self.faces
            .get(idx)
            .ok_or(MeshError::IndexingError("Indexing failed."))
    }

    /**
    Gets a face normal at a given index.

    Parameters:
    - `idx: usize` - The index of the face normal to retrieve.

    Returns:
    - `Result<&UnitVec3, MeshError>` - Returns `&UnitVec3` if the indexing succeeds, else
    `MeshError::IndexingError`.
     */
    pub fn get_normal(&self, idx: usize) -> Result<&UnitVec3, MeshError> {
        self.face_normals
            .get(idx)
            .ok_or(MeshError::IndexingError("Indexing failed."))
    }

    /// Adds `vertex: Point3` to the mesh and returns the index where the vertex will reside.
    fn add_vertex(&mut self, vertex: Point3) -> usize {
        self.vertices.push(vertex);
        self.vertices.len() - 1
    }

    /**
    Attempts to add a face (and optionally the associated normal) to the mesh. If a normal is not
    given, it will be calculated by taking the cross-product of `face[1] - face[0]` and
    `face[2] - face[0]`. This method assumes the points are in counter-clockwise order and that the
    requested vertices are co-planar.

    Parameters:
    - `face: Vec<usize>` - The vertices to be added as a face.
    - `face_normal: Option<UnitVec3>` - The normal for the face.

    Returns:
    - `Result<usize, MeshError>` - Returns the index at which the face and its associated normal were
    added.
     */
    fn add_face(
        &mut self,
        face: Vec<usize>,
        face_normal: Option<UnitVec3>,
    ) -> Result<usize, MeshError> {
        if let Some(normal) = face_normal {
            self.face_normals.push(normal);
        } else {
            self.face_normals.push(match get_face_normal(self, &face) {
                Ok(t) => t,
                Err(e) => return Err(e),
            });
        }

        self.faces.push(face);
        Ok(self.faces.len() - 1)
    }
}
