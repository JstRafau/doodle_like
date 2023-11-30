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
    speed: f64,
    angular_speed: f64,
    poz: bool,
    #[base]
    sprite: Base<Sprite2D>
}
impl Player {
    fn pos (&mut self) {
        self.sprite.translate(Vector2::new(200.0, 300.0));
        self.poz = false;
    }
}
#[godot_api]
impl ISprite2D for Player {
    fn init(sprite: Base<Sprite2D>) -> Self {
        godot_print!("Hello, world!"); // Prints to the Godot console
        Self {
            speed: 400.0,
            angular_speed: std::f64::consts::PI,
            poz: true,
            sprite
        }
    }
    fn physics_process(&mut self, delta: f64) {
        // In GDScript, this would be: 
        // rotation += angular_speed * delta
        if self.poz {
            self.pos();
        }
        godot_print!("Hello, world!: {delta}"); // Prints to the Godot console
        self.sprite.rotate((self.angular_speed * delta) as f32);
        // The 'rotate' method requires a f32, 
        // therefore we convert 'self.angular_speed * delta' which is a f64 to a f32
        if Input::singleton().is_action_pressed("mv_right".into()) {
            self.sprite.translate(Vector2::RIGHT);
        };
    }
}
