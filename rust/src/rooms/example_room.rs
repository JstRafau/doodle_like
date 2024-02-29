use godot::{
    prelude::*,
    engine::{
        INode2D,
        Node2D,
        NodeExt,
        Polygon2D,
        TileMap,
    }
};


#[derive(GodotClass)]
#[class(base=Node2D)]
struct ExampleRoom {
    #[base]
    base: Base<Node2D>,
    is_cleared: bool,
}

#[godot_api]
impl ExampleRoom {
    #[func]
    pub fn new_game(&mut self) {
        let mut player = self.base.get_node_as
            ::<crate::character::PlayerCharacter>("Character");

        player.bind_mut().start();
    }
    #[func]
    fn check_enemies(&mut self) {
        let nodes = self.base.get_tree().unwrap().get_nodes_in_group("enemy".into());
        let mut enemy_counter: u8 = 0;
        for _item in nodes.iter_shared(){
            enemy_counter += 1;
        }
        match enemy_counter {
            0 => {
                self.is_cleared = true;
                godot_print!("Killed 'em all");
                // temporary
                self.base
                    .get_tree()
                    .unwrap()
                    .call_group(
                        "run".into(),
                        "player_died".into(),
                        &["gg game".to_variant()]
                        );
            },
            _ => (),
        }
    }
}
#[godot_api]
impl INode2D for ExampleRoom {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            base,
            is_cleared: false,
        }
    }

    fn ready(&mut self) {
        /*
        let room_tiles = self.base.get_node_as::<TileMap>("TileMap");
        let mut polygon = self.base.get_node_as::<Polygon2D>("Desk");
        let mut desk_tiles = room_tiles.get_used_cells(1);
        let mut test: PackedVector2Array = PackedVector2Array::new();
        for _ in 0..=9 {
            let item = desk_tiles.pop().unwrap();
            /*
            if item.y > 0 {
                continue;
            }
            */
            test.push(Vector2::new(item.x as real * 20.0 + 40.0, item.y as real * 20.0 + 40.0));
            test.push(Vector2::new(item.x as real * 20.0 - 40.0, item.y as real * 20.0 + 40.0));
            test.push(Vector2::new(item.x as real * 20.0 + 40.0, item.y as real * 20.0 - 40.0));
            break;
        }
        
        polygon.set_polygon(test);
        godot_print!("{:?}", desk_tiles);
        */
        //
        let viewport = self.base.get_viewport_rect();
        let position = Vector2::new(viewport.size.x / 2., viewport.size.y / 2.);
        self.base.set_global_position(position);
    }
    fn process(&mut self, _delta: f64) {
        if !self.is_cleared {
            self.check_enemies();
        }
    }
}

