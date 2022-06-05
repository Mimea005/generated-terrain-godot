#![forbid(non_fmt_panics, missing_docs)]
//! terrgen is a collection of generators
//! to generate terrain in a godot scene
use gdnative::init::*;

mod camera;
mod generators;
mod math;

fn init(register: InitHandle) {

    register.add_class::<generators::flat::FlatTerrain>();
    register.add_class::<camera::CameraController>();
    register.add_class::<generators::basic::BasicTerrain>();

}

godot_init!(init);
