use godot::{prelude::*, engine::{PhysicsBody2D, CollisionShape2D, AnimatedSprite2D, CharacterBody2D, ICharacterBody2D}};


#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Bob {
    name: String,
    hit_points: u8,
    speed: real,
    damage: f64,
    sprite_atlas: String,
    #[base]
    base: Base<CharacterBody2D>,
}

#[godot_api]
impl Bob {
    #[func]
    fn on_player_body_entered(&mut self, _body: Gd<PhysicsBody2D>) {
        let mut collision_shape = self.base
            .get_node_as::<CollisionShape2D>("CollisionShape2D");
        collision_shape.set_deferred("disabled".into(), true.to_variant());
        //let mut animated_sprite = self.base.get_node_as::<Sprite2D>("Sprite2D");
    }
    
    #[func]
    pub fn start(&mut self) {
        self.base.show();

        let mut collision_shape = self
            .base
            .get_node_as::<CollisionShape2D>("CollisionShape2D");

        collision_shape.set_disabled(false);
    }
}

#[godot_api]
impl ICharacterBody2D for Bob {
    fn init(base: Base<CharacterBody2D>) -> Self {
        Self {
            name: String::from("Bob"), 
            hit_points: 5,
            speed: 200.,
            damage: 10.,
            sprite_atlas: String::from("test"),
            base
        }
    }

    fn ready(&mut self) {
    }

    fn physics_process(&mut self, _delta: f64) {
        let input_dir: Vector2 = Input::singleton().get_vector(
            "mv_left".into(),
            "mv_right".into(),
            "mv_up".into(),
            "mv_down".into(),
        );
        let velocity = input_dir.normalized() * self.speed;

        let mut animated_sprite = self
            .base
            .get_node_as::<AnimatedSprite2D>("AnimatedSprite2DBody");

        if velocity.length() > 0.0 {

            self.base.set_velocity(velocity);
            self.base.move_and_slide();

            let animation;

            if velocity.x != 0. || velocity.y != 0. {
                animation = "walk";
                animated_sprite.set_flip_h(velocity.x > 0.0);
            } else {
                animation = "stand";
            }

            animated_sprite.play_ex().name(animation.into()).done();
        } else {
            animated_sprite.play_ex().name("stand".into()).done();
            animated_sprite.stop();
        }
    }
    
    fn process(&mut self, _delta: f64) {
    }
}
