use godot::{
    engine::{
        Control,
        Node2D,
        INode2D,
    }, prelude::*
};


#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct DDLRun {
    game_over: Gd<PackedScene>,
    game_ending: Gd<PackedScene>,
    levels: [Gd<PackedScene>; 3],
    quick_menu: Gd<PackedScene>,
    #[base]
    base: Base<Node2D>,
}

#[godot_api]
impl DDLRun {
    #[func]
    fn quick_menu(&mut self) {
        let quick_menu = self.quick_menu.instantiate_as::<Control>();
        self.base.add_child(quick_menu.clone().upcast());
        self.base.get_tree().unwrap().set_pause(true);
    }

    #[func]
    fn player_died(&mut self) {
        let mut game_over_scene = self.game_over.instantiate_as::<Control>();
        let viewport = self.base.get_viewport_rect();
        let position = Vector2::new(viewport.size.x / 2., viewport.size.y / 2.);
        game_over_scene.set_global_position(position);
        self.base.add_child(game_over_scene.clone().upcast());

        self.base.get_tree().unwrap().set_pause(true);
    }

    #[func]
    fn player_won(&mut self) {
        self.game_ending = load("res://scenes/cutscenes/ending.tscn");
    }

    #[func]
    fn jump_to_end_scene(&mut self) {
        self.base.get_tree().unwrap().change_scene_to_packed(self.game_ending.clone());
    }
}

#[godot_api]
impl INode2D for DDLRun {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            game_over: PackedScene::new(),
            game_ending: PackedScene::new(),
            levels: [
                PackedScene::new(),
                PackedScene::new(),
                PackedScene::new()
            ],
            quick_menu: PackedScene::new(),
            base, 
        }
    }
    fn ready(&mut self) {
        self.game_over = load("res://scenes/gui/game_over/game_over.tscn");
        self.quick_menu = load("res://scenes/gui/quick_menu/quick_menu.tscn");
        self.levels[0] = load("res://scenes/level.tscn");
        let mut level = self.levels[0].instantiate_as::<Node2D>();
        let viewport = self.base.get_viewport_rect();
        let position = Vector2::new(viewport.size.x / 2., viewport.size.y / 2.);
        level.set_position(position);
        level.set_meta("floor".into(), 1.to_variant());
        self.base.add_child(level.upcast());
    }

    fn process(&mut self, _delta: f64) {
        if Input::singleton().is_action_just_pressed("ui_cancel".into()) {
            self.quick_menu();
        }
    }
}

