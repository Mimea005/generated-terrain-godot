use gdextras::*;
use gdnative::prelude::*;
use gdnative::api::{
    MeshInstance,
    ArrayMesh,
    Texture
};
use crate::math::common::Lerp;
use rand::{Rng, SeedableRng};

#[derive(NativeClass)]
#[inherit(MeshInstance)]
#[register_with(Self::register)]
pub struct BasicTerrain {

    /// Sizes of a terrain segment
    chunk_width: i32,
    chunk_depth: i32,

    chunk_data: Vec<Vec<f32>>

}


impl BasicTerrain {

    fn new(_owner: TRef<MeshInstance>) -> Self {

        Self {
            chunk_width: 10,
            chunk_depth: 10,
            chunk_data: Vec::new()
        }
    }

    fn register(builder: &ClassBuilder<Self>) {

        builder.property("Width")
            .with_getter(|s,_|s.chunk_width)
            .with_setter(|s,_, v: i32| s.chunk_width = v)
            .with_default(10)
            .done();

        builder.property("Depth")
            .with_getter(|s,_|s.chunk_depth)
            .with_setter(|s,_, v: i32| s.chunk_depth = v)
            .with_default(10)
            .done();
    }
}

#[methods]
impl BasicTerrain {

    pub fn generate_noise_map(&mut self) {

        let mut map = vec![vec![0.; self.chunk_depth as usize + 2]; self.chunk_width as usize + 2];

        let mut rng = rand::rngs::SmallRng::seed_from_u64(10);

        for x in 0..map.len() {
            for y in 0..map[0].len() {
                map[x][y] = rng.gen_range(0_f32..1.);
            }
        }

        self.chunk_data = map;

    }

    pub fn get_terrain_height(&self, pos: Vector2) -> f32 {

        //  Find the lowest point to round to
        let snap_pos = Vector2::new(
            pos.x.floor(),
            pos.y.floor()
        );

        //  get terrain height for the 4 points around pos
        let p1 = self.chunk_data[snap_pos.x as usize][snap_pos.y as usize];
        let p2 = self.chunk_data[snap_pos.x as usize + 1][snap_pos.y as usize];
        let p3 = self.chunk_data[snap_pos.x as usize][snap_pos.y as usize + 1];
        let p4 = self.chunk_data[snap_pos.x as usize + 1][snap_pos.y as usize + 1];

        //  Get the percentage between points pos represents
        let percent = Vector2::new(
            snap_pos.x.lerp_inv(snap_pos.x+1., pos.x),
            snap_pos.y.lerp_inv(snap_pos.y, snap_pos.y+1.)
        );

        //  Interpolate on x axis
        let interp0 = p1.lerp(p2, percent.x);
        let interp1 = p3.lerp(p4, percent.x);

        //  Return interpolation between the two x axis values
        interp0.lerp(interp1, percent.y)
    }

    pub fn get_terrain_slope(&self, pos1: Vector2, pos2: Vector2) -> f32 {

        //  Get height
        let h1 = self.get_terrain_height(pos1);
        let h2 = self.get_terrain_height(pos2);

        //  Get the height difference
        let height_delta = h1 - h2;
        let pos_delta = pos1.distance_to(pos2);

        height_delta/pos_delta
    }


    /// generate mesh based on chunk_data and resolution
    pub fn generate_terrain_mesh(&self, _owner: TRef<MeshInstance>) -> Ref<ArrayMesh, Unique> {


        //  Generate terrain vertices

        let mut vertices = Vec::new();
        let mut uvs = Vec::new();

        for x in 0..self.chunk_width + 1 {
            for y in 0..self.chunk_depth + 1 {

                vertices.push( Vector3::new(
                    x as f32,
                    self.chunk_data[x as usize][y as usize],
                    y as f32
                ));

                uvs.push(Vector2::new(
                   x as f32/self.chunk_width as f32,
                    y as f32 / self.chunk_depth as f32
                ));
            }
        }

        let mut indeces = Vec::new();

        //  Generate triangles
        let mut vert = 0;
        for _ in 0..self.chunk_width {
            for _ in 0..self.chunk_depth {

                //  First triangle
                indeces.push(vert);
                indeces.push(vert + self.chunk_depth + 1);
                indeces.push(vert + 1);

                //  Second triangle
                indeces.push(vert + 1);
                indeces.push(vert + self.chunk_depth + 1);
                indeces.push(vert + self.chunk_depth + 2);

                vert+=1;
            }

            vert += 1;
        }

        //  Generate mesh

        let arrays = VariantArray::new();
        arrays.resize(9);
        arrays.set(0, Vector3Array::from_vec(vertices));    //  Vertices
        arrays.set(4, Vector2Array::from_vec(uvs)); //  UV map
        arrays.set(8, Int32Array::from_vec(indeces));   //  Vertex indexes

        let arr_mesh = ArrayMesh::new();

        arr_mesh.add_surface_from_arrays(4, arrays.into_shared(), VariantArray::new().into_shared(), 2194432);

        arr_mesh
    }

    #[export]
    fn _ready(&mut self, owner: TRef<MeshInstance>) {

        self.generate_noise_map();

        owner.set_mesh(self.generate_terrain_mesh(owner));

    }
}
