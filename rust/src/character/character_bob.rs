use godot::{prelude::*, engine::{Sprite2D, PhysicsBody2D, CollisionShape2D, AnimatedSprite2D}};


#[derive(GodotClass)]
#[class(base=Node2D)]
struct Bob {
    name: String,
    hit_points: u8,
    speed: real,
    damage: f64,
    sprite_atlas: String,
    #[base]
    base: Base<Node2D>,
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
        let viewport = self.base.get_viewport_rect();
        self.base.set_global_position(Vector2::new(viewport.size.x / 2., viewport.size.y / 2.));
        self.base.show();

        let mut collision_shape = self
            .base
            .get_node_as::<CollisionShape2D>("CollisionShape2D");

        collision_shape.set_disabled(false);
    }
}

#[godot_api]
impl INode2D for Bob {
    fn init(base: Base<Node2D>) -> Self {
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
    fn process(&mut self, delta: f64) {
        let mut animated_sprite = self
            .base
            .get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");

        let mut velocity = Vector2::new(0.0, 0.0);

        let input = Input::singleton();
        if input.is_action_pressed("mv_right".into()) {
            velocity += Vector2::RIGHT;
        }
        if input.is_action_pressed("mv_left".into()) {
            velocity += Vector2::LEFT;
        }
        if input.is_action_pressed("mv_down".into()) {
            velocity += Vector2::DOWN;
        }
        if input.is_action_pressed("mv_up".into()) {
            velocity += Vector2::UP;
        }

        if velocity.length() > 0.0 {
            velocity = velocity.normalized() * self.speed;

            let animation;

            if velocity.x > 0. || velocity.y > 0. {
                animation = "walk";
            } else {
                animation = "stand";
            }

            animated_sprite.play_ex().name(animation.into()).done();
        } else {
            animated_sprite.stop();
        }

        let change = velocity * real::from_f64(delta);
        let position = self.base.get_global_position() + change;
        let viewport = self.base.get_viewport_rect();
        let position = Vector2::new(
            position.x.clamp(viewport.size.x / 15. + 30., viewport.size.x / 15. * 14. - 30.),
            position.y.clamp(viewport.size.y / 9. + 40., viewport.size.y / 9. * 8. - 40.),
        );
        self.base.set_global_position(position);
    }
}
