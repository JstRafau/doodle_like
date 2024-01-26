use godot::{prelude::*, engine::{INode2D, Node2D}};


#[derive(GodotClass)]
#[class(base=Node2D)]
struct ExampleRoom {
    #[base]
    base: Base<Node2D>
}

#[godot_api]
impl ExampleRoom {
    #[func]
    pub fn new_game(&mut self) {
        let mut player = self.base.get_node_as
            ::<crate::character::PlayerCharacter>("Character");

        player.bind_mut().start();
    }
}
#[godot_api]
impl INode2D for ExampleRoom {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            base
        }
    }

    fn ready(&mut self) {
        let viewport = self.base.get_viewport_rect();
        let position = Vector2::new(viewport.size.x / 2., viewport.size.y / 2.);
        self.base.set_global_position(position);
    }
}

