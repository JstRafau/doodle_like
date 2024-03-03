use godot::{
    prelude::*,
    engine::{
        INode2D,
        Node2D,
    }
};


#[derive(GodotClass)]
#[class(base=Node2D)]
struct EntranceRoom {
    #[base]
    base: Base<Node2D>,
    is_cleared: bool,
}

#[godot_api]
impl EntranceRoom {
    #[func]
    pub fn new_game(&mut self) {
        godot_print!("temp");
    }
}

#[godot_api]
impl INode2D for EntranceRoom {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            base,
            is_cleared: true,
        }
    }

    fn ready(&mut self) {
        let viewport = self.base.get_viewport_rect();
        let position = Vector2::new(viewport.size.x / 2., viewport.size.y / 2.);
        self.base.set_global_position(position);
    }
    fn process(&mut self, _delta: f64) {
    }
}

