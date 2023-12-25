use godot::{
    prelude::*,
    engine::{
        AnimatedSprite2D,
        Area2D,
        CollisionShape2D,
        CharacterBody2D,
        ICharacterBody2D,
        PhysicsBody2D,
    },
};



#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct PlayerCharacter {
    pub name: String,
    pub hit_points: u8,
    pub speed: real,
    pub damage: f64,
    #[base]
    base: Base<CharacterBody2D>,
    bullet: Gd<PackedScene>,
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

    #[func]
    fn shoot(&mut self, shoot_direction: Vector2) {
        let mut bullet_scene = self.bullet.instantiate_as::<Area2D>();
        bullet_scene.set_position(self.base.get_position());

        /*
         *  Make some code that can translate Vector2 to rotation
         *  might have to use match for every posibility
         *  (but I'd rather don't that)
        */

        let mut owner = self.base.get_owner().unwrap();
        owner.add_child(bullet_scene.clone().upcast());

        bullet_scene.set_global_rotation(shoot_direction.angle());
    }
}

#[godot_api]
impl ICharacterBody2D for PlayerCharacter {
    fn init(base: Base<CharacterBody2D>) -> Self {
        Self {
            name: String::from("Placeholder_Name"), 
            hit_points: 1,
            speed: 350.,
            damage: 10.,
            base,
            bullet: PackedScene::new(),
        }
    }

    fn ready(&mut self) {
        self.bullet = load("res://scenes/projectile.tscn");
        godot_print!("{}", self.name);
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
        let shoot_direction: Vector2 = Input::singleton().get_vector(
            "aim_left".into(),
            "aim_right".into(),
            "aim_up".into(),
            "aim_down".into(),
        );

        if shoot_direction.length() > 0. {
            self.shoot(shoot_direction);
        }
    }
}
