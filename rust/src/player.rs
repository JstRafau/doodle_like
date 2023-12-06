use godot::{
    prelude::*,
    engine::{
        Sprite2D,
        ISprite2D,
    },
};


#[derive(GodotClass)]
#[class(base=Sprite2D)]
struct Player {
    hit_points: u8,
    speed: f32,
    #[base]
    sprite: Base<Sprite2D>
}

#[godot_api]
impl ISprite2D for Player {
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
    fn ready(&mut self) {
        self.sprite.set_position(Vector2::new(640.0, 360.0));
        //                               ^__^^^  ^__^^^
        //  Magic numbers. Replace them with screen_width/2
        //  or something like that :-)
        self.hit_points -= 1;
    }
    fn physics_process(&mut self, delta: f64) {
        let current_pos = self.sprite.get_position();
        let mut new_pos: (f32, f32) = (current_pos.x, current_pos.y);
        if Input::singleton().is_action_pressed("mv_right".into()) {
            //
            //  This whole validation will be unnecesary as soon as I add hitboxes.
            //  It also aplies to orher move inputs.
            //
            new_pos.0 = match new_pos.0 as i32 {
                80..=1200 => new_pos.0 + (self.speed * delta as f32),
                _ => new_pos.0,
            };
        };
        if Input::singleton().is_action_pressed("mv_left".into()) {
            new_pos.0 = match current_pos.x as i32 {
                80..=1280 => new_pos.0 - (self.speed * delta as f32),
                _ => new_pos.0,
            };
        };
        if Input::singleton().is_action_pressed("mv_up".into()) {
            new_pos.1 = match new_pos.1 as i32 {
                200..=720 => new_pos.1 - (self.speed * delta as f32),
                _ => new_pos.1,
            };
        };
        if Input::singleton().is_action_pressed("mv_down".into()) {
            new_pos.1 = match new_pos.1 as i32 {
                0..=520 => new_pos.1 + (self.speed * delta as f32),
                _ => new_pos.1,
            };
        };
        
        self.sprite.set_position(Vector2::new(new_pos.0, new_pos.1));
    }
}
