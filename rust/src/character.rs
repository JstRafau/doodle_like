use godot::{
    prelude::*,
    engine::{
        AnimatedSprite2D,
        Area2D,
        CollisionShape2D,
        CharacterBody2D,
        ICharacterBody2D,
    },
};


#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct PlayerCharacter {
    pub name: String,
    pub hit_points: (u8, f64),
    pub speed: real,
    pub damage: f64,
    #[base]
    base: Base<CharacterBody2D>,
    bullet: Gd<PackedScene>,
}

#[godot_api]
impl PlayerCharacter {
    
    #[func]
    pub fn start(&mut self) {
        self.base.show();

        let mut collision_shape = self
            .base
            .get_node_as::<CollisionShape2D>("CollisionShape2D");

        collision_shape.set_disabled(false);
    }

    #[func]
    fn on_player_body_entered(&mut self, body: Gd<Node2D>) {
        if body.is_in_group("player".into()) {
            return;
        }

        let mut projectile_hit_shape = self.base
            .get_node_as::<CollisionShape2D>("ProjectileHitDetector/ProjectileCollisionShape2D");
        let mut physical_hit_shape = self.base
            .get_node_as::<CollisionShape2D>("PhysicalHitDetector/PhysicalCollisionShape2D");


        godot_warn!("Ouchie!");
        self.change_color_on_damage(1., 0.69, 0.69);

        self.hit_points.0 -= 1;

        if self.hit_points.0 == 0 {
            godot_error!("No moar hp!!!");
            self.change_color_on_damage(1., 0.95, 0.95);
            self.update_sprite("died".into(), false);
            //,______________________________,
            //|           yuo dead           |
            //|      display some stats      |
            //|                              |
            //|      [restart_game_btn]      |
            //|      [back_to_menu_btn]      |
            //|______________________________|
        }

        physical_hit_shape.set_deferred("disabled".into(), true.to_variant());
        projectile_hit_shape.set_deferred("disabled".into(), true.to_variant());
        self.hit_points.1 = 0.;
    }

    #[func]
    fn update_hp_timeout(&mut self, delta: f64) {
        if self.hit_points.1 >= 1. {
            return;
        } 

        let sprite_modulation = self.base
            .get_node_as::<AnimatedSprite2D>("AnimatedSprite2DBody")
            .get_modulate();

        if sprite_modulation.g < 1. {
            self.change_color_on_damage(
                1.,
                sprite_modulation.g + 0.005,
                sprite_modulation.b + 0.005
            );
        }
        
        self.hit_points.1 = if (self.hit_points.1 + delta) < 1. {
            self.hit_points.1 + delta
        } else {
            self.base
                .get_node_as::<CollisionShape2D>("ProjectileHitDetector/ProjectileCollisionShape2D")
                .set_deferred("disabled".into(), false.to_variant());
            self.base
                .get_node_as::<CollisionShape2D>("PhysicalHitDetector/PhysicalCollisionShape2D")
                .set_deferred("disabled".into(), false.to_variant());
            // no semicolon means that if statement returns 1.0
            1.
        };
    }

    #[func]
    fn shoot(&mut self, mut shoot_direction: Vector2) {
        let mut bullet_scene = self.bullet.instantiate_as::<Area2D>();
        bullet_scene.set_position(self.base.get_position());

        shoot_direction += self.get_normalized_movement_vector() * 0.6;

        bullet_scene.set_global_rotation(shoot_direction.angle());
        let mut owner = self.base.get_owner().unwrap();
        owner.add_child(bullet_scene.clone().upcast());
    }

    #[func]
    fn get_normalized_movement_vector(&mut self) -> Vector2 {
        let input_dir: Vector2 = Input::singleton().get_vector(
            "mv_left".into(),
            "mv_right".into(),
            "mv_up".into(),
            "mv_down".into(),
        );

        input_dir.normalized()
    }

    #[func]
    fn update_sprite(&mut self, animation: StringName, flip: bool) {
        let mut animated_sprite = self
            .base
            .get_node_as::<AnimatedSprite2D>("AnimatedSprite2DBody");
        match animation.to_string().as_str() {
            "stand" => animated_sprite.stop(),
            _ => (),
        }
        animated_sprite.play_ex().name(animation.into()).done();
        animated_sprite.set_flip_h(flip);
    }

    #[func]
    fn change_color_on_damage(&mut self, r: f32, g: f32, b: f32) {
        let mut animated_sprite = self
            .base
            .get_node_as::<AnimatedSprite2D>("AnimatedSprite2DBody");
        animated_sprite.set_modulate(Color::from_rgb(r, g, b));
    }
}

#[godot_api]
impl ICharacterBody2D for PlayerCharacter {
    fn init(base: Base<CharacterBody2D>) -> Self {
        Self {
            name: String::from("Placeholder_Name"), 
            hit_points: (1, 1.),
            speed: 350.,
            damage: 10.,
            base,
            bullet: PackedScene::new(),
        }
    }

    fn ready(&mut self) {
        //self.hit_points.0 
        let hp = self.base.get_meta("hp".into());
        self.hit_points.0 = hp.to(); 
        self.bullet = load("res://scenes/projectile.tscn");
        godot_print!("{}", self.name);
    }

    fn physics_process(&mut self, delta: f64) {
        if self.hit_points.0 == 0 {
            return;
        }
        let input_velocity = self.get_normalized_movement_vector();
        let mut audio_walk = self.base.get_node_as::<AudioStreamPlayer>("WalkAudio");
        audio_walk.set_bus("aaa".into());
        let playing = audio_walk.is_playing();

        if input_velocity.length() > 0.0 {
            if !playing {
                audio_walk.play();
            }

            let current_velocity = self.base
                .get_velocity()
                .move_toward(input_velocity * self.speed, delta as f32 * self.speed * 12.);
            godot_print!("{}", current_velocity);
            self.base.set_velocity(current_velocity);
            self.base.move_and_slide();

            let animation: &str;
            let mut flip: bool = false;

            if input_velocity.y < 0. {
                animation = "walk_up";
                flip = input_velocity.x > 0.0;
            } else if input_velocity.x != 0. || input_velocity.y > 0. {
                animation = "walk";
                flip = input_velocity.x > 0.0;
            } else {
                animation = "stand";
            }

            self.update_sprite(animation.into(), flip);
        } else {
            let current_velocity = self.base.get_velocity();
            if current_velocity.length() > 0. {
                godot_print!("-> {}", current_velocity);
                self.base.set_velocity(current_velocity
                    .move_toward(Vector2::ZERO, delta as f32 * self.speed * 12.));
                return;
            }
            self.update_sprite("stand".into(), false);
            audio_walk.stop();
        }
    }
    
    fn process(&mut self, delta: f64) {
        if self.hit_points.0 == 0 {
            return;
        }
        self.update_hp_timeout(delta);

        let mut shoot: bool = false; 
        for i in ["aim_left", "aim_right", "aim_up", "aim_down"] {
            if Input::singleton().is_action_just_pressed(i.into()) {
                shoot = true;
                break;
            }
        }
        if shoot {
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
}
