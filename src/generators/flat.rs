use gdnative::prelude::*;
use gdnative::api::{
    MeshInstance,
    ArrayMesh,
};

#[derive(NativeClass)]
#[inherit(MeshInstance)]
#[register_with(Self::register)]
pub struct FlatTerrain {

    /// Sizes of a terrain segment
    chunk_width: i32,
    chunk_depth: i32

}


impl FlatTerrain {

    fn new(_owner: TRef<MeshInstance>) -> Self {
        Self {
            chunk_width: 10,
            chunk_depth: 10,
        }
    }

    fn register(builder: &ClassBuilder<Self>) {

        builder.property("Width")
            .with_getter(|s,_|s.chunk_width)
            .with_setter(|s,_, v: i32| s.chunk_width = v)
            .with_default(10)
            .done();

        builder.property("Height")
            .with_getter(|s,_|s.chunk_depth)
            .with_setter(|s,_, v: i32| s.chunk_depth = v)
            .with_default(10)
            .done();
    }
}

#[methods]
impl FlatTerrain {

    /// generate mesh based on chunk_data and resolution
    pub fn generate_terrain_mesh(&self) -> Ref<ArrayMesh, Unique> {


        //  Generate terrain vertices

        let mut vertices: Vec<Vector3> = Vec::new();

        for x in 0..self.chunk_width + 1 {
            for y in 0..self.chunk_depth + 1 {

                vertices.push( Vector3::new(
                    x as f32,
                    0.,
                    y as f32
                ));
            }
        }

        let mut invert_index = Vec::new();

        //  Generate triangles
        let mut vert = 0;
        for _ in 0..self.chunk_width {
            for _ in 0..self.chunk_depth {

                //  First triangle
                invert_index.push(vert);
                invert_index.push(vert + 1);
                invert_index.push(vert + self.chunk_depth + 1);

                //  Second triangle
                invert_index.push(vert + 1);
                invert_index.push(vert + self.chunk_depth + 2);
                invert_index.push(vert + self.chunk_depth + 1);

                vert+=1;
            }

            vert += 1;
        }

        let mut indeces = Vec::new();
        for _ in 0..invert_index.len() {indeces.push(invert_index.pop().unwrap())}

        //  Generate mesh

        let arrays = VariantArray::new();
        arrays.resize(9);
        arrays.set(0, Vector3Array::from_vec(vertices));
        arrays.set(8, Int32Array::from_vec(indeces));

        let arr_mesh = ArrayMesh::new();

        arr_mesh.add_surface_from_arrays(4, arrays.into_shared(), VariantArray::new().into_shared(), 2194432);

        arr_mesh
    }

    #[export]
    fn _ready(&self, owner: TRef<MeshInstance>) {

        owner.set_mesh(self.generate_terrain_mesh())

    }
}
