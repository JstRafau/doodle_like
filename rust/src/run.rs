use godot::{
    prelude::*,
    engine::{
        Control,
        Node2D,
        INode2D,
    },
};



#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct Run {
    game_over: Gd<PackedScene>,
    #[base]
    base: Base<Node2D>,
    character: Gd<PackedScene>,
}

#[godot_api]
impl Run {
    #[func]
    fn player_died(&mut self) {
        let mut game_over_scene = self.game_over.instantiate_as::<Control>();
        let viewport = self.base.get_viewport_rect();
        let position = Vector2::new(viewport.size.x / 2., viewport.size.y / 2.);
        game_over_scene.set_global_position(position);
        self.base.add_child(game_over_scene.clone().upcast());

        self.base
            .find_child("ExampleRoom".into())
            .unwrap()
            .get_tree()
            .unwrap()
            .set_pause(true);
    }
}

#[godot_api]
impl INode2D for Run {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            game_over: PackedScene::new(),
            base, 
            character: PackedScene::new(),
        }
    }
    fn ready(&mut self) {
        self.game_over = load("res://scenes/gui/game_over/game_over.tscn");
    }

    // fn process(&mut self, _delta: f64) {}
}

