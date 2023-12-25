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
}


#[godot_api]
impl IArea2D for Projectile {
    fn init(base: Base<Area2D>) -> Self {
        Self {
            speed: 600.,
            damage: 10.,
            base
        }
    }

    fn ready(&mut self) {
    }

    fn physics_process(&mut self, delta: f64) {
        let rotation = self.base.get_rotation();
        let position = self.base.get_position()
            + Vector2::RIGHT.rotated(rotation) * self.speed * delta as real;
        
        self.base.set_position(position);
    }
}
