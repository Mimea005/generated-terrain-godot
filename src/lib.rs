//! terrgen is a collection of generators
//! to generate terrain in a godot scene
use gdnative::init::*;
use gdnative::prelude::*;
use gdnative::api::*;
use gdextras::*;

mod camera;
mod generators;
mod math;


fn init(register: InitHandle) {

    register.add_class::<generators::flat::FlatTerrain>();
    register.add_class::<camera::CameraController>();

}

godot_init!(init);
