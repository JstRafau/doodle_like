use godot::{
    prelude::*,
    engine::{
        Node2D,
        INode2D,
        Polygon2D,
        TileMap,
    }
};


#[derive(GodotClass)]
#[class(base=Node2D)]
struct DDLBaseRoom {
    #[base]
    base: Base<Node2D>,
    is_cleared: bool,
}

#[godot_api]
impl DDLBaseRoom {
    #[func]
    fn check_enemies(&mut self) {
        let nodes = self.base.get_tree().unwrap().get_nodes_in_group("enemy".into());
        let enemy_count = nodes.iter_shared().count();

        match enemy_count {
            0 => {
                self.is_cleared = true;
                // temporary
                let mut tree = self.base
                    .get_tree()
                    .unwrap();
                tree.call_group(
                    "run".into(),
                    "player_won".into(),
                    &[/*"gg game".to_variant()*/]
                );
                tree.call_group(
                    "run".into(),
                    "jump_to_end_scene".into(),
                    &[/*"gg game".to_variant()*/]
                );
            },
            _ => (),
        }
    }

    #[func]
    fn open_door(&self, side: u8) {
        /*  *  *  *  *  *  *  *  *  *  *  *  *  *  *  *  *  *  *  *  *
         *  A node will block doors, and after calling this function,
         *  it'll play an animation of it being erased,
         *  together with disabling it's hitbox.
         *  *  *  *  *  *  *  *  *  *  *  *  *  *  *  *  *  *  *  *  */ 
    }
}
#[godot_api]
impl INode2D for DDLBaseRoom {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            base,
            is_cleared: false,
        }
    }

    fn ready(&mut self) {
        self.is_cleared = self.base.get_meta("is_cleared".into()).to();
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
    }
    fn process(&mut self, _delta: f64) {
        if !self.is_cleared {
            self.check_enemies();
        }
    }
}

