use gdnative::prelude::*;
use gdnative::api::{
    InputEventMouseMotion,
    Camera
};
use gdextras::*;
use gdnative::export::hint::{FloatHint, RangeHint};

#[derive(NativeClass)]
#[inherit(Spatial)]
#[register_with(Self::register)]
pub struct CameraController {

    sensitivity: f32,

    captured: bool
}

#[methods]
impl CameraController {

    fn new(_owner: TRef<Spatial>) -> Self {
        Self {
            sensitivity: 0.0001,
            captured: false
        }
    }

    fn register(builder: &ClassBuilder<Self>) {

        builder.property("Mouse sensitivity")
            .with_setter(|s,_,v|
                s.sensitivity = v / 100_000.)
            .with_getter(|s,_|
                s.sensitivity * 100_000.)
            .with_default(10.)
            .with_hint(FloatHint::Range(RangeHint::new(0.1,100.).with_step(0.1)))
            .done();

    }

    #[export]
    fn _input(&mut self, owner: TRef<Spatial>, event: Ref<InputEvent>) {

        let event = unsafe { event.assume_safe() };

        if event.as_text().to_string().starts_with("InputEventMouseMotion") && self.captured {

            let event = event.cast::<InputEventMouseMotion>().unwrap();

            let mut motion = event.relative();

            motion.x *= self.sensitivity;
            motion.y *= self.sensitivity;

            owner.global_rotate(Vector3::UP, motion.x as f64);
            owner.rotate_object_local(Vector3::LEFT, motion.y as f64);

        }
        else if event.is_action_pressed("ui_accept", false, false) {

            if self.captured {
                self.captured = false;
                Input::godot_singleton().set_mouse_mode(0);
                gd_print!(owner, p, "release")
            }
            else {
                self.captured = true;
                Input::godot_singleton().set_mouse_mode(2);
                gd_print!(owner, p, "capture")
            }
        }

        let camera: TRef<Camera> = get_node(owner.clone(), "Camera").unwrap();

        if event.is_action_pressed("zoom_in", false, false) {
            camera.translate_object_local(Vector3::FORWARD)
        }
        else if event.is_action_pressed("zoom_out", false, false) {
            camera.translate_object_local(Vector3::BACK)
        }

    }

    #[export]
    fn _unhandled_input(&mut self, owner: TRef<Spatial>, event: Ref<InputEvent>) {

        let event = unsafe { event.assume_safe() };

        if event.is_action_pressed("ui_cancel", false, false) {
            unsafe {
                owner.get_tree().unwrap().assume_safe().quit(0)
            }
        }

    }
}
