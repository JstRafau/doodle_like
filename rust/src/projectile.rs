use godot::{
    prelude::*,
    engine::{
        Area2D,
        IArea2D,
    },
};


#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct Projectile {
    pub speed: real,
    pub damage: f64,
    move_direction: Vector2,
    #[base]
    base: Base<Area2D>,
}


#[godot_api]
impl Projectile {
    #[func]
    fn on_projectile_body_entered(&mut self, mut body: Gd<Area2D>) {
        if body.is_in_group("enemy".into()) {
            body.queue_free();
        }
        self.base.queue_free();
    }

    #[func]
    pub fn set_direction(&mut self, shoot_direction: Vector2) {
        self.move_direction = shoot_direction;
    }
}


#[godot_api]
impl IArea2D for Projectile {
    fn init(base: Base<Area2D>) -> Self {
        Self {
            speed: 600.,
            damage: 10.,
            move_direction: Vector2::RIGHT,
            base
        }
    }

    fn ready(&mut self) {
    }

    fn physics_process(&mut self, delta: f64) {
        let position = self.base.get_position()
            + self.move_direction * self.speed * delta as real;
        
        self.base.set_position(position);
    }
}
