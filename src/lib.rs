pub mod geometry;

use nalgebra::{Unit, Vector3};

type Float = f32;
type Int = i32;
type Uint = u32;
type Vec3 = Vector3<Float>;
type UnitVec3 = Unit<Vec3>;
type Point3 = nalgebra::Point3<Float>;

#[cfg(test)]
mod tests {
    use crate::geometry::mesh::*;
    use crate::Vec3;
    use nalgebra::{vector, Point3};

    /// Test Valid OBJ files to ensure expected result is retrieved.
    #[test]
    fn test_valid_obj_loader() {
        assert!(PolygonMesh::load_obj("test-files/octahedron.obj").is_ok());
        let mesh = PolygonMesh::load_obj("test-files/octahedron.obj")
            .ok()
            .unwrap();
        assert_eq!(mesh.get_vertex_count(), 6);
        assert_eq!(mesh.get_face_count(), 8);

        // Check vertices
        assert_eq!(
            *mesh.get_vertex(0).ok().unwrap(),
            Point3::from([1., 0., 0.])
        );
        assert_eq!(
            *mesh.get_vertex(1).ok().unwrap(),
            Point3::from([0., -1., 0.])
        );
        assert_eq!(
            *mesh.get_vertex(2).ok().unwrap(),
            Point3::from([-1., 0., 0.])
        );
        assert_eq!(
            *mesh.get_vertex(3).ok().unwrap(),
            Point3::from([0., 1., 0.])
        );
        assert_eq!(
            *mesh.get_vertex(4).ok().unwrap(),
            Point3::from([0., 0., 1.])
        );
        assert_eq!(
            *mesh.get_vertex(5).ok().unwrap(),
            Point3::from([0., 0., -1.])
        );

        // Check faces
        assert_eq!(*mesh.get_face(0).ok().unwrap(), vec![1, 0, 4]);
        assert_eq!(*mesh.get_face(1).ok().unwrap(), vec![2, 1, 4]);
        assert_eq!(*mesh.get_face(2).ok().unwrap(), vec![3, 2, 4]);
        assert_eq!(*mesh.get_face(3).ok().unwrap(), vec![0, 3, 4]);
        assert_eq!(*mesh.get_face(4).ok().unwrap(), vec![0, 1, 5]);
        assert_eq!(*mesh.get_face(5).ok().unwrap(), vec![1, 2, 5]);
        assert_eq!(*mesh.get_face(6).ok().unwrap(), vec![2, 3, 5]);
        assert_eq!(*mesh.get_face(7).ok().unwrap(), vec![3, 0, 5]);

        // Check normals
        assert_eq!(
            mesh.get_normal(0).ok().unwrap().into_inner(),
            Vec3::normalize(&vector![1., -1., 1.])
        );
        assert_eq!(
            mesh.get_normal(1).ok().unwrap().into_inner(),
            Vec3::normalize(&vector![-1., -1., 1.])
        );
        assert_eq!(
            mesh.get_normal(2).ok().unwrap().into_inner(),
            Vec3::normalize(&vector![-1., 1., 1.])
        );
        assert_eq!(
            mesh.get_normal(3).ok().unwrap().into_inner(),
            Vec3::normalize(&vector![1., 1., 1.])
        );
        assert_eq!(
            mesh.get_normal(4).ok().unwrap().into_inner(),
            Vec3::normalize(&vector![1., -1., -1.])
        );
        assert_eq!(
            mesh.get_normal(5).ok().unwrap().into_inner(),
            Vec3::normalize(&vector![-1., -1., -1.])
        );
        assert_eq!(
            mesh.get_normal(6).ok().unwrap().into_inner(),
            Vec3::normalize(&vector![-1., 1., -1.])
        );
        assert_eq!(
            mesh.get_normal(7).ok().unwrap().into_inner(),
            Vec3::normalize(&vector![1., 1., -1.])
        );

        assert!(PolygonMesh::load_obj("test-files/trumpet.obj").is_ok());
    }

    /// Test Invalid OBJ files and ensure errors are as expected.
    #[test]
    fn test_invalid_obj_loader() {
        assert!(
            PolygonMesh::load_obj("test-files/invalid_objs/invalid-prefix.obj")
                .err()
                .map_or(
                    false,
                    |x| matches!(x, MeshError::FormatError(x) if x == "Invalid file line.")
                )
        );
        assert!(
            PolygonMesh::load_obj("test-files/invalid_objs/invalid-float.obj")
                .err()
                .map_or(
                    false,
                    |x| matches!(x, MeshError::FormatError(x) if x == "Failed to parse float.")
                )
        );
        assert!(
            PolygonMesh::load_obj("test-files/invalid_objs/invalid-integer.obj")
                .err()
                .map_or(
                    false,
                    |x| matches!(x, MeshError::FormatError(x) if x == "Failed to parse integer.")
                )
        );
        assert!(
            PolygonMesh::load_obj("test-files/invalid_objs/invalid-string.obj")
                .err()
                .map_or(
                    false,
                    |x| matches!(x, MeshError::FormatError(x) if x == "Unable to process string.")
                )
        );
        assert!(
            PolygonMesh::load_obj("test-files/invalid_objs/non-existent-file.obj")
                .err()
                .map_or(
                    false,
                    |x| matches!(x, MeshError::IOError(x) if x == "File not found.")
                )
        );
        assert!(PolygonMesh::load_obj("test-files/invalid_objs/invalid-indexing.obj").err().map_or(
            false,
            |x| matches!(x, MeshError::IndexingError(x) if x == "Vertex not contained in mesh.")
        ));
        assert!(PolygonMesh::load_obj("/root").err().map_or(
            false,
            |x| matches!(x, MeshError::IOError(x) if x == "Insufficient permissions.")
        ));
        assert!(PolygonMesh::load_obj("/").err().map_or(
            false,
            |x| matches!(x, MeshError::IOError(x) if x == "Could not read next line.")
        ));
    }

    #[test]
    fn write_obj() {
        let f = PolygonMesh::load_obj("test-files/octahedron.obj")
            .ok()
            .unwrap()
            .write_obj("test-files/write-octahedron.obj");

        assert!(f.is_ok());
        assert_eq!(f.unwrap(), 115);
    }
}
