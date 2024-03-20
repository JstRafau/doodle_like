use godot::{
    engine::utilities::randi,
    prelude::*,
};


#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct DDLLevel {
    #[base]
    base: Base<Node2D>,
    map: [[bool; 11]; 11],
}


#[godot_api]
impl DDLLevel {
    //#[func]
    fn fill_map(&mut self) -> [[bool; 11]; 11] {
        let mut room_pos_selector: Vector2i = Vector2i::new(5, 5);
        let mut new_map: [[bool; 11]; 11] = [[false; 11]; 11];
        new_map[room_pos_selector.x as usize][room_pos_selector.y as usize] = true;

        let floor: u8 = self.base.get_meta("floor".into()).to();

        for i in 0..=(floor + 3) {
            loop {
                let direction: i64 = randi().abs() % 4;
                match direction {
                    0 => room_pos_selector.x -= 1,
                    1 => room_pos_selector.y += 1,
                    2 => room_pos_selector.x += 1,
                    _ => room_pos_selector.y -= 1,
                };
                if !new_map[room_pos_selector.x as usize][room_pos_selector.y as usize] {
                    new_map[room_pos_selector.x as usize][room_pos_selector.y as usize] = true;
                    break;
                }
            }
        }

        for j in 0..=10 {
            let mut row_of_rooms: String = String::new();
            for i in 0..=10 {
                if new_map[i][j] {
                    row_of_rooms.push_str("#");
                } else {
                    row_of_rooms.push_str(".");
                }
            }
            godot_print!("{}", row_of_rooms);
        }

        new_map
    }

    fn place_room(&mut self) -> Vec<Vector2i> {
        let center: Vector2i = Vector2i::new(5, 5);
        let mut rooms_coords_relative: Vec<Vector2i> = Vec::new();
        for i in 0..=10 {
            for j in 0..=10 {
                match self.map[i][j] {
                    false => (),
                    true => {
                        rooms_coords_relative.push(Vector2i::new(
                                i as i32 - center.x,
                                j as i32 - center.y
                        ));
                    },
                }
            }
        }
        rooms_coords_relative
    }

    #[func]
    fn add_room_scene(&mut self, coord: Vector2i) {
        let mut new_room_packed = PackedScene::new();
        new_room_packed = load("res://scenes/rooms/room_1_02.tscn");
        let mut new_room = new_room_packed
            .instantiate_as::<Node2D>();
        new_room.set_transform(
            Transform2D::IDENTITY.translated_local(
                Vector2::new(
                    1280.0 * coord.x as f32,
                    720.0 * coord.y as f32
        )));
        self.base.add_child(new_room.upcast());
    }

    #[func]
    fn shift_rooms(&mut self, dir: Vector2) { 
        let viewport = self.base.get_viewport_rect();
        let current_transform = self.base.get_transform();
        let map_shift: Transform2D = current_transform.translated_local(
            Vector2::new(
                viewport.size.x * dir.x,
                viewport.size.y * dir.y
        ));
        self.base.set_transform(map_shift);

        self.base.get_tree().unwrap()
            .get_nodes_in_group("projectile".into()).iter_shared()
            .for_each(|mut node| node.queue_free());
        //  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        // It is disturbing and beautiful at the same time
    }
}

    #[godot_api]
    impl INode2D for DDLLevel {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            base,
            map: [[false; 11]; 11],
        }
    }

    fn ready(&mut self) {
        self.map = self.fill_map();
        let rooms = self.place_room();
        rooms.iter().for_each(|coord|
            if coord.x != 0 || coord.y != 0 {
            self.add_room_scene(coord.clone())
        });
    }

    fn process(&mut self, _delta: f64) {
    }
}
