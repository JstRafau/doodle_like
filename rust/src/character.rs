use godot::{
    engine::{
        AnimatedSprite2D,
        Area2D,
        CharacterBody2D,
        ICharacterBody2D,
        CollisionShape2D,
        NodeExt,
        Timer,
    },
    prelude::*,
};

use crate::DEFAULTS;


#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct DDLPlayerCharacter{
    pub name: String,
    pub hit_points: (u8, u8, f64),
    pub speed: real,
    pub shoot_speed: f64,
    pub damage: f64,
    player_coords: Vector2i,
    #[base]
    base: Base<CharacterBody2D>,
    bullet: Gd<PackedScene>,
}

#[godot_api]
impl DDLPlayerCharacter{
    // signals
    #[signal]
    fn hit();

    #[func]
    fn pass_coords(&self){
        self.base.get_tree().unwrap()
            .call_group("room".into(), "has_player_entered".into(), &[self.player_coords.to_variant()]);
    }

    // functions
    #[func]
    pub fn check_room_change(&mut self) {
        let pos = self.base.get_position();
        if pos.y > 640.0 {
            self.base.set_velocity(Vector2::ZERO);
            self.base.set_position(Vector2::new(pos.x, 60.0));
            self.base
                .get_tree()
                .unwrap()
                .call_group(
                    "level".into(),
                    "shift_rooms".into(),
                    &[Vector2::new(0.0, -1.0).to_variant()]
                );
            self.player_coords.y += 1;
            self.pass_coords();
            return;
        }
        if pos.y < 40.0 {
            self.base.set_velocity(Vector2::ZERO);
            self.base.set_position(Vector2::new(pos.x, 600.0));
            self.base
                .get_tree()
                .unwrap()
                .call_group(
                    "level".into(),
                    "shift_rooms".into(),
                    &[Vector2::new(0.0, 1.0).to_variant()]
                );
            self.player_coords.y -= 1;
            self.pass_coords();
            return;
        }
        if pos.x > 1170.0 {
            self.base.set_velocity(Vector2::ZERO);
            self.base.set_position(Vector2::new(140.0, pos.y));
            self.base
                .get_tree()
                .unwrap()
                .call_group(
                    "level".into(),
                    "shift_rooms".into(),
                    &[Vector2::new(-1.0, 0.0).to_variant()]
                );
            self.player_coords.x += 1;
            self.pass_coords();
            return;
        }
        if pos.x < 110.0 {
            self.base.set_velocity(Vector2::ZERO);
            self.base.set_position(Vector2::new(1120.0, pos.y));
            self.base
                .get_tree()
                .unwrap()
                .call_group(
                    "level".into(),
                    "shift_rooms".into(),
                    &[Vector2::new(1.0, 0.0).to_variant()]
                );
            self.player_coords.x -= 1;
            self.pass_coords();
            return;
        }
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
    fn on_player_body_entered(&mut self, body: Gd<Node2D>) {
        if body.is_in_group("player".into()) {
            return;
        }

        let mut audio_taken_damage = self.base.get_node_as::<AudioStreamPlayer>("DamageAudio");
        audio_taken_damage.set_bus("aaa".into());
        audio_taken_damage.play();

        self.change_color_on_damage(1., 0.69, 0.69);

        self.hit_points.0 -= 1;

        self.base.get_tree().unwrap()
            .call_group("hud".into(), "update_hp".into(),&[
                self.hit_points.0.to_variant(),
                self.hit_points.1.to_variant()
            ]); 

        if self.hit_points.0 == 0 {
            self.change_color_on_damage(1., 0.95, 0.95);
            self.update_sprite("died".into(), false);
            self.base
                .get_tree()
                .unwrap()
                .call_group(
                    "run".into(),
                    "player_died".into(),
                    &[self.name.to_variant()]
                );
        }

        let mut projectile_hit_shape = self.base
            .get_node_as::<CollisionShape2D>("ProjectileHitDetector/ProjectileCollisionShape2D");
        let mut physical_hit_shape = self.base
            .get_node_as::<CollisionShape2D>("PhysicalHitDetector/PhysicalCollisionShape2D");

        physical_hit_shape.set_deferred("disabled".into(), true.to_variant());
        projectile_hit_shape.set_deferred("disabled".into(), true.to_variant());
        self.hit_points.2 = 0.;
    }

    #[func]
    fn update_hp_timeout(&mut self, delta: f64) {
        if self.hit_points.2 >= 1. {
            return;
        } 

        let sprite_modulation = self.base
            .get_node_as::<AnimatedSprite2D>("AnimatedSprite2DBody")
            .get_modulate();

        if sprite_modulation.g < 1. {
            self.change_color_on_damage(
                1.,
                sprite_modulation.g + delta as f32,
                sprite_modulation.b + delta as f32
            );
        }
        
        self.hit_points.2 = if (self.hit_points.2 + delta) < 1. {
            self.hit_points.2 + delta
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
        let mut shoot_timer = self.base.get_node_as::<Timer>("CanShootTimer");

        if shoot_timer.get_time_left() > 0.0 {
            return;
        }
        shoot_timer.start();
        let mut bullet_scene = self.bullet.instantiate_as::<Area2D>();
        bullet_scene.set_position(self.base.get_position());

        shoot_direction += self.get_normalized_movement_vector() * 0.3;

        bullet_scene.set_global_rotation(shoot_direction.angle());
        let mut run = self.base.get_tree().unwrap()
            .get_first_node_in_group("run".into()).unwrap();
        run.add_child(bullet_scene.clone().upcast());

        let mut audio_shoot = self.base.get_node_as::<AudioStreamPlayer>("ShootAudio");
        audio_shoot.set_bus("aaa".into());
        audio_shoot.play();
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
impl ICharacterBody2D for DDLPlayerCharacter{
    fn init(base: Base<CharacterBody2D>) -> Self {
        Self {
            name: String::from("Placeholder_Name"), 
            hit_points: (1, 1, 1.),
            speed: DEFAULTS.speed,
            shoot_speed: 0.35,
            damage: 10.,
            player_coords: Vector2i::new(0, 0),
            base,
            bullet: PackedScene::new(),
        }
    }

    fn ready(&mut self) {
        self.name = self.base.get_meta("name".into()).to();
        self.hit_points.0 = self.base.get_meta("hp".into()).to(); 
        self.hit_points.1 = self.base.get_meta("hp".into()).to(); 
        self.damage = self.base.get_meta("damage".into()).to(); 
        let speed: f32 = self.base.get_meta("speed".into()).to(); 
        self.speed *= speed;
        let shoot_speed: f32 = self.base.get_meta("shoot_speed".into()).to();
        self.shoot_speed = shoot_speed as f64;
        let viewport = self.base.get_viewport_rect();
        let position = Vector2::new(viewport.size.x / 2., viewport.size.y / 2.);
        self.base.set_global_position(position);

        let mut shoot_timer = self.base.get_node_as::<Timer>("CanShootTimer");
        shoot_timer.set_wait_time(self.shoot_speed);

        self.bullet = load("res://scenes/projectile.tscn");
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
            self.base.set_velocity(current_velocity);
            self.base.move_and_slide();
            
            self.check_room_change();

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
                self.base.set_velocity(current_velocity
                    .move_toward(Vector2::ZERO, delta as f32 * self.speed * 12.));
                self.base.move_and_slide();
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
            if Input::singleton().is_action_pressed(i.into()) {
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
