use godot::{
    prelude::*,
    engine::{
        Control,
        Node2D,
        INode2D,
        TileMap,
    },
};

use crate::character::PlayerCharacter;


#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct Run {
    game_over: Gd<PackedScene>,
    #[base]
    base: Base<Node2D>,
}

#[godot_api]
impl Run {
    /*
    #[func]
    fn on_new_game_start(&self) {
        let hud_hp = self.base.get_node_as::<TileMap>("HUD/TileMap");
        let mut player = self.base.get_node_as::<PlayerCharacter>("Bob");
        player.connect("hit".into(), hud_hp.callable("update_hp"));
    }
    */

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
        }
    }
    fn ready(&mut self) {
        self.game_over = load("res://scenes/gui/game_over/game_over.tscn");
    }

    fn process(&mut self, _delta: f64) {}
}

