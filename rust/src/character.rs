use godot::{prelude::*, engine::{PhysicsBody2D, CollisionShape2D, AnimatedSprite2D, CharacterBody2D, ICharacterBody2D}};


#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct PlayerCharacter {
    name: String,
    hit_points: u8,
    speed: real,
    damage: f64,
    #[base]
    base: Base<CharacterBody2D>,
}

#[godot_api]
impl PlayerCharacter {
    #[func]
    fn on_player_body_entered(&mut self, _body: Gd<PhysicsBody2D>) {
        let mut collision_shape = self.base
            .get_node_as::<CollisionShape2D>("CollisionShape2D");
        collision_shape.set_deferred("disabled".into(), true.to_variant());
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
impl ICharacterBody2D for PlayerCharacter {
    fn init(base: Base<CharacterBody2D>) -> Self {
        Self {
            name: String::from("Bob"), 
            hit_points: 5,
            speed: 350.,
            damage: 10.,
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
        let mut audio_walk = self.base.get_node_as::<AudioStreamPlayer>("WalkAudio");
        audio_walk.set_bus("aaa".into());
        let playing = audio_walk.is_playing();

        if velocity.length() > 0.0 {
            if !playing {
                audio_walk.play();
            }

            self.base.set_velocity(velocity);
            self.base.move_and_slide();

            let animation;

            if velocity.y < 0. {
                animation = "walk_up";
                animated_sprite.set_flip_h(velocity.x > 0.0);
            } else if velocity.x != 0. || velocity.y > 0. {
                animation = "walk";
                animated_sprite.set_flip_h(velocity.x > 0.0);
            } else {
                animation = "stand";
            }

            animated_sprite.play_ex().name(animation.into()).done();
        } else {
            animated_sprite.play_ex().name("stand".into()).done();
            audio_walk.stop();
            animated_sprite.stop();
        }
    }
    
    fn process(&mut self, _delta: f64) {

    }
}
