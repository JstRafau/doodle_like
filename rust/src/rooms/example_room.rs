use godot::{prelude::*, engine::{IStaticBody2D, StaticBody2D}};


#[derive(GodotClass)]
#[class(base=StaticBody2D)]
struct ExampleRoom {
    #[base]
    base: Base<StaticBody2D>
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
impl IStaticBody2D for ExampleRoom {
    fn init(base: Base<StaticBody2D>) -> Self {
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

