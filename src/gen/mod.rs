use amethyst::{
    renderer::rendy::{
        mesh::*,
        hal::Primitive,
    },
};
use ncollide3d::procedural::TriMesh;
use log::info;
use rand::prelude::*;
use rand_distr::{Standard, LogNormal, Uniform, UnitCircle, Exp1, Float};
use obj_exporter::{Geometry, ObjSet, Object, Primitive as ObjPrimitive, Shape, TVertex, Vertex};
use terr::{
    heightmap::{Heightmap, Voronoi, diamond_square, fault_displacement},
    unbounded::Perlin,
};

#[derive(Clone, Copy)]
pub enum Terrain {
    Mountains,
    Foothills,
    Coast,
    Fault,
}

impl Distribution<Terrain> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Terrain {
        match rng.gen_range(0, 4) {
            0 => Terrain::Mountains,
            1 => Terrain::Foothills,
            2 => Terrain::Coast,
            _ => Terrain::Fault,
        }
    }
}

pub struct MapGenerator {
    map_type: Terrain,
    mesh: TriMesh<f32>,
    path: String,
}

impl MapGenerator {
    pub fn new(map_type: Terrain) -> Self {
        match map_type {
            Terrain::Mountains => {
                let cells = 2_u32.pow(5) + 1;
                let mut heightmap = Heightmap::new_flat((cells, cells), (100.0, 100.0));

                // Try different weights and numbers of points!
                let w = [-0.8, 0.2, 0.4];
                // let w = [-0.9, 1.2];
                // let w = [0.7, -1.2];
                let voronoi = Voronoi::random(&heightmap, 24, &mut rand::thread_rng());
                voronoi.apply_to(&mut heightmap, &w, |x,y| (x*x + y*y).sqrt());

                let mut quad = heightmap.to_trimesh();
                for p in &mut quad.coords {
                    // Quad is created with z=height, but y is up in amethyst.
                    // We must rotate all three coords to keep the right side up.
                    let temp = p.z;
                    p.z = p.x;
                    p.x = p.y;
                    p.y = temp;
                }
                quad.recompute_normals();
                info!("Terrain generation finished");

                let num: usize = rand::thread_rng().gen();
                MapGenerator {
                    map_type: map_type,
                    mesh: quad,
                    path: format!("assets/prefabs/map-{}.ron", num),
                }
            },
            
            Terrain::Foothills => {
                let cells = 2_u32.pow(6) + 1;
                let mut heightmap = Heightmap::new_flat((cells, cells), (100.0, 100.0));

                // Randomise the height of the four corners:
                let distr = LogNormal::new(0.5, 1.0).unwrap();
                let mut rng = rand::thread_rng();
                for (x, y) in [(0, 0), (0, cells-1), (cells-1, 0), (cells-1, cells-1)].iter() {
                    let h = distr.sample(&mut rng) as f32;
                    heightmap.set(*x, *y, h);
                }

                // Note: Normal(0, scale) is possibly better, but not yet available for f32.
                let scale = 0.1;
                let distr = Uniform::new(-scale, scale);
                diamond_square(&mut heightmap, 0, &mut rng, distr).unwrap();

                let w = [-1.0, 0.5, 1.0];
                let voronoi = Voronoi::random(&heightmap, 24, &mut rand::thread_rng());
                voronoi.apply_to(&mut heightmap, &w, |x,y| 0.01 * (x*x + y*y));

                let mut quad = heightmap.to_trimesh();
                for p in &mut quad.coords {
                    // Quad is created with z=height, but y is up in amethyst.
                    // We must rotate all three coords to keep the right side up.
                    let temp = p.z;
                    p.z = p.x;
                    p.x = p.y;
                    p.y = temp;
                }
                quad.recompute_normals();
                info!("Terrain generation finished");

                let num: usize = rand::thread_rng().gen();
                MapGenerator {
                    map_type: map_type,
                    mesh: quad,
                    path: format!("assets/prefabs/map-{}.ron", num),
                }
            },
            
            Terrain::Coast => {
                let mut rng = thread_rng();

                let cells = 2_u32.pow(8);
                let mut heightmap = Heightmap::new_flat((cells, cells), (100.0, 100.0));
                let mut ampl = 20.0;
                let mut larc = 1.0 / (cells as f32);
                for _ in 0..7 {
                    let sampler = || {
                        let g: [f32; 2] = UnitCircle.sample(&mut rng);
                        let s: f32 = Exp1.sample(&mut rng);
                        [g[0] * s, g[1] * s]
                    };
                    let surface = Perlin::new(larc, 1024, sampler).unwrap();
                    heightmap.add_surface(&surface, ampl);
                    ampl *= 0.5;
                    larc *= 2.0;
                }

                let mut quad = heightmap.to_trimesh();
                for p in &mut quad.coords {
                    // Quad is created with z=height, but y is up in amethyst.
                    // We must rotate all three coords to keep the right side up.
                    let temp = p.z;
                    p.z = p.x;
                    p.x = p.y;
                    p.y = temp;
                }
                quad.recompute_normals();
                info!("Terrain generation finished");

                let num: usize = rand::thread_rng().gen();
                MapGenerator {
                    map_type: map_type,
                    mesh: quad,
                    path: format!("assets/prefabs/map-{}.ron", num),
                }
            },
            
            Terrain::Fault => {
                let cells = 2_u32.pow(6) + 1; // must be 2.powi(n) + 1 for some integer n
                let mut heightmap = Heightmap::new_flat((cells, cells), (100.0, 100.0));

                // Randomise the height of the four corners:
                let distr = LogNormal::new(0.5, 1.0).unwrap();
                let mut rng = rand::thread_rng();
                for (x, y) in [(0, 0), (0, cells-1), (cells-1, 0), (cells-1, cells-1)].iter() {
                    let h = distr.sample(&mut rng) as f32;
                    heightmap.set(*x, *y, h);
                }

                // Perform random midpoint displacement with randomised scale.
                let scale = LogNormal::new(-2.5, 0.5).unwrap().sample(&mut rng) as f32;
                // Note: Normal(0, scale) is possibly better, but not yet available for f32.
                let distr = Uniform::new(-scale, scale);
                diamond_square(&mut heightmap, 0, &mut rng, distr).unwrap();

                let n_faults = rng.sample(LogNormal::new(1.5, 0.5).unwrap()) as usize;
                let r_dist = LogNormal::new(2.0, 1.0).unwrap();
                for _ in 0..n_faults {
                    let r = rng.sample(r_dist) as f32;
                    let h = 0.1 * r;
                    fault_displacement(&mut heightmap, &mut rng, (0.0, r), |d| {
                        if d >= 0.0 && d < r {
                            h * (1.0 - (d / r).powi(2)).powi(2)
                        } else {
                            0.0
                        }
                    });
                }

                let mut quad = heightmap.to_trimesh();
                for p in &mut quad.coords {
                    // Quad is created with z=height, but y is up in kiss3d's camera.
                    // We must rotate all three coords to keep the right side up.
                    let temp = p.z;
                    p.z = p.x;
                    p.x = p.y;
                    p.y = temp;
                }
                quad.recompute_normals();
                info!("Terrain generation finished");

                let num: usize = rand::thread_rng().gen();
                MapGenerator {
                    map_type: map_type,
                    mesh: quad,
                    path: format!("assets/prefabs/map-{}.ron", num),
                }
            },
        }
    }

    // .obj generation
    pub fn build_mesh(&self) {
        let num: usize = rand::thread_rng().gen();
        let rpath = format!("terrain-{}.obj", num);
        let mesh_object = ObjSet {
            material_library: None,
            objects: vec![Object {
                name: String::from("Terrain"),
                vertices: self.mesh.coords
                    .clone()
                    .into_iter()
                    .map(|point| {
                        let slice: [f32; 3] = point.coords.into();

                        Vertex { x: slice[0] as f64, y: slice[1] as f64, z: slice[2] as f64 }
                    })
                    .collect(),
                tex_vertices: self.mesh.uvs
                    .clone()
                    .expect("[ERROR][raiders::gen] UV vector not found")
                    .into_iter()
                    .map(|point| {
                        let slice: [f32; 2] = point.coords.into();

                        TVertex { u: slice[0] as f64, v: slice[1] as f64, w: 0.0 }
                    })
                    .collect(),
                normals: self.mesh.normals
                    .clone()
                    .expect("[ERROR][raiders::gen] Normals vector not found")
                    .into_iter()
                    .map(|vector| {
                        let slice: [f32; 3] = vector.into();

                        Vertex { x: slice[0] as f64, y: slice[1] as f64, z: slice[2] as f64 }
                    })
                    .collect(),
                geometry: vec![Geometry {
                    material_name: None,
                    shapes: self.mesh.indices
                        .clone()
                        .unwrap_unified()
                        .into_iter()
                        .map(|point| {
                            let slice: [u32; 3] = point.coords.into();

                            Shape {
                                primitive: ObjPrimitive::Triangle(
                                    (slice[0] as usize, Some(slice[0] as usize), Some(0)),
                                    (slice[1] as usize, Some(slice[1] as usize), Some(0)),
                                    (slice[2] as usize, Some(slice[2] as usize), Some(0)),
                                ),
                                groups: vec![],
                                smoothing_groups: vec![],
                            }
                        })
                        .collect(),
                }],
            }],
        };

        obj_exporter::export_to_file(&mesh_object, rpath).unwrap();
    }

    // MeshBuilder constructor (doesn't work)
    pub fn _build_mesh(&self) -> MeshBuilder<'static> {
        let coords: Vec<PosNorm> = self.mesh.coords
            .clone()
            .into_iter()
            .map(|point| {
                let slice: [f32; 3] = point.coords.into();
                let position: Position = slice.into();
                let normal: Normal = slice.into();

                PosNorm { position, normal }
            })
            .collect();

        let uvs: Vec<TexCoord> = self.mesh.uvs
            .clone()
            .expect("[ERROR][raiders::gen] UV vector not found")
            .into_iter()
            .map(|point| {
                let slice: [f32; 2] = point.coords.into();
                let uv: TexCoord = slice.into();

                uv
            })
            .collect();

        let verts: Vec<PosNormTex> = coords.into_iter()
            .zip(uvs.into_iter())
            .map(|(posnorm, tex_coord)| {
                let PosNorm { position, normal } = posnorm;

                PosNormTex { position, normal, tex_coord }
            })
            .collect();

        let mut indices: Vec<u32> = vec![];
        self.mesh.indices
            .clone()
            .unwrap_unified()
            .into_iter()
            .for_each(|point| {
                let slice: [u32; 3] = point.coords.into();

                for x in slice.iter() {
                    indices.push(x.clone());
                }
            });

        MeshBuilder::from(verts)
            .with_indices(indices)
            .with_prim_type(Primitive::TriangleList)
    }

    pub fn map_type(&self) -> Terrain { self.map_type }
    pub fn map_path(&self) -> &str { self.path.as_str() }
}
