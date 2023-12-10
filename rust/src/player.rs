use godot::{
    prelude::*,
};


#[derive(GodotClass)]
//#[class(base=Sprite2D)]
#[class(base=Node2D)]
struct Player {
    hit_points: u8,
    speed: f64,
    damage: f64,
    sprite_atlas: String,
    #[base]
    node: Base<Node2D>,
//    #[base]
//    sprite: Base<Sprite2D>
}

#[godot_api]
impl INode2D for Player {
    /*
    fn init(sprite: Base<Sprite2D>) -> Self {
        godot_print!("Hello, world!"); // Prints to the Godot console
        Self {
            //
            //  Those values should be (preferably) held
            //  in a separate JSON/TOML/YML file.
            //
            hit_points: 5,
            speed: 200.0,
            sprite
        }
    }
    */
    fn init(node: Base<Node2D>) -> Self {
        godot_warn!("player does nothing on init");
        Self {
            hit_points: 0,
            speed: 0.,
            damage: 0.,
            sprite_atlas: String::from("test"),
            node
        }
    }

    fn ready(&mut self) {
            //.set_position(Vector2::new(640.0, 360.0));
        //                               ^__^^^  ^__^^^
        //  Magic numbers. Replace them with screen_width/2
        //  or something like that :-)
        self.node.translate(Vector2::new(64.0, 64.0));
        godot_warn!("player does nothing on ready");
    }

    fn physics_process(&mut self, delta: f64) {
        if Input::singleton().is_action_pressed("mv_right".into()) {
            godot_print!("D");
            self.node.translate(Vector2::new(64.0 * delta as f32, 64.0 * delta as f32));
        };
        if Input::singleton().is_action_pressed("mv_left".into()) {
            godot_print!("A");
        };
        if Input::singleton().is_action_pressed("mv_up".into()) {
            godot_print!("W");
        };
        if Input::singleton().is_action_pressed("mv_down".into()) {
            godot_print!("S");
        };
    }
}
