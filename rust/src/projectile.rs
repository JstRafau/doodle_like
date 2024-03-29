use godot::{
    prelude::*,
    engine::{
        Area2D,
        IArea2D,
        CollisionShape2D,
    },
};

use crate::DEFAULTS;


#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct DDLProjectile {
    pub speed: real,
    pub damage: f64,
    #[base]
    base: Base<Area2D>,
}


#[godot_api]
impl DDLProjectile {
    #[func]
    fn set_collision(&mut self) {
        let mut collision_shape = self
            .base
            .get_node_as::<CollisionShape2D>("CollisionShape2D");

        collision_shape.set_disabled(false);
    }

    #[func]
    fn on_bullet_body_entered(&mut self, mut body: Gd<Node2D>) {
        let true_variant: Variant = true.to_variant();
        if  self.base.get_meta("hostile".into()).ne(&true_variant) && body.is_in_group("enemy".into()) {
            body.queue_free();
        }
        self.base.queue_free();
    }
}


#[godot_api]
impl IArea2D for DDLProjectile {
    fn init(base: Base<Area2D>) -> Self {
        Self {
            speed: DEFAULTS.speed,
            damage: 10.,
            base
        }
    }

    fn ready(&mut self) {
        self.set_collision();
        let speed: f32 = self.base.get_meta("speed".into()).to();
        self.speed *= speed;
    }

    fn physics_process(&mut self, delta: f64) {
        let rotation = self.base.get_rotation();
        let position = Vector2::RIGHT.rotated(rotation) * self.speed * delta as real;
        let transform = self.base.get_transform();
        self.base.set_transform(transform.translated(position));
    }
}
