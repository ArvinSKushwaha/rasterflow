# RasterFlow

![GitHub Workflow Status](https://img.shields.io/github/workflow/status/ArvinSKushwaha/rasterflow/Rust?style=for-the-badge)
![GitHub](https://img.shields.io/github/license/ArvinSKushwaha/rasterflow?style=for-the-badge)
![GitHub commit activity](https://img.shields.io/github/commit-activity/w/ArvinSKushwaha/rasterflow?style=for-the-badge)

High-Performance fluid simulation and visualization. This project mostly exists to become more familiar with Rust, and
to build a project that could one day become applicable to my interests in Computational Fluid Dynamics.

## Progress

- [x] Implement `PolygonMesh` object to handle mesh input for fluids.
- [x] Implement OBJ processing to load `PolygonMesh` objects from .OBJ files
- [x] Use abstraction to generalize `PolygonMesh` and implement `TriangleMesh`
- [ ] Implement `Tetrahedron` and `TetrahedralMesh` structs.
  - [x] Implement `Tetrahedron`
  - [ ] Implement `TetrahedralMesh`
- [ ] Add abstraction by implementing `Cell` and `CellMesh` traits.
- [ ] Implement Discretizer trait
