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
            speed: 100.0,
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
        if Input::singleton().is_action_pressed("mv_right".into()) {
            //
            //  This whole validation will be unnecesary as soon as I add hitboxes.
            //  It also aplies to orher move inputs.
            //
            let new_pos: f32 = match current_pos.x as i32 {
                80..=1200 => current_pos.x + (self.speed * delta as f32),
                _ => current_pos.x,
            };
            self.sprite.set_position(Vector2::new(new_pos, current_pos.y));
            godot_print!("RIGHT");
        };
        if Input::singleton().is_action_pressed("mv_left".into()) {
            let new_pos: f32 = match current_pos.x as i32 {
                80..=1280 => current_pos.x - (self.speed * delta as f32),
                _ => current_pos.x,
            };
            self.sprite.set_position(Vector2::new(new_pos, current_pos.y));
            godot_print!("LEFT");
        };
        if Input::singleton().is_action_pressed("mv_up".into()) {
            let new_pos: f32 = match current_pos.y as i32 {
                200..=720 => current_pos.y - (self.speed * delta as f32),
                _ => current_pos.y,
            };
            self.sprite.set_position(Vector2::new(current_pos.x, new_pos));
            godot_print!("UP");
        };
        if Input::singleton().is_action_pressed("mv_down".into()) {
            let new_pos: f32 = match current_pos.y as i32 {
                0..=520 => current_pos.y + (self.speed * delta as f32),
                _ => current_pos.y,
            };
            self.sprite.set_position(Vector2::new(current_pos.x, new_pos));
            godot_print!("DOWN");
        };
    }
}
