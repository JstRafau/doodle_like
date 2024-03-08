use godot::{
    prelude::*,
    engine::{
        INode2D,
        Node2D,
    }
};


#[derive(GodotClass)]
#[class(base=Node2D)]
struct DDLEntranceRoom {
    #[base]
    base: Base<Node2D>,
}

#[godot_api]
impl DDLEntranceRoom {
    #[func]
    pub fn new_game(&mut self) {
        godot_print!("temp");
    }
}

#[godot_api]
impl INode2D for DDLEntranceRoom {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            base,
        }
    }

    fn ready(&mut self) {
    }

    fn process(&mut self, _delta: f64) {
    }
}

