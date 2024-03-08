use godot::{engine::CenterContainer, prelude::*};


#[derive(Copy, Clone)]
struct RoomData {
    room_id: u8, 
    is_cleared: bool,
}

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct DDLLevel {
    #[base]
    base: Base<Node2D>,
    map: [[bool; 11]; 11],
    player_coords: (u8, u8),
}


#[godot_api]
impl DDLLevel {
    //#[func]
    fn fill_map(&mut self) -> [[bool; 11]; 11] {
        let center = (5, 5);
        let mut new_map: [[bool; 11]; 11] = [[false; 11]; 11];

        new_map[center.0][center.1 + 1] = true;
        new_map[center.0][center.1 + 2] = true;
        new_map[center.0 + 1][center.1 + 2] = true;

        new_map
    }

    fn place_room(&mut self) -> Vec<(i8, i8)> {
        let center = (5, 5);
        let mut rooms_coords_relative: Vec<(i8, i8)> = Vec::new();
        for i in 0..=10 {
            for j in 0..=10 {
                match self.map[i][j] {
                    false => (),
                    true => {
                        rooms_coords_relative.push((i as i8 - center.0, j as i8 - center.1));
                    },
                }
            }
        }
        godot_print!("{:?}", rooms_coords_relative);
        rooms_coords_relative
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
            player_coords: (5, 5),
        }
    }

    fn ready(&mut self) {
        self.map = self.fill_map();
        let rooms = self.place_room();
        let mut room01_pack = PackedScene::new();
        room01_pack = load("res://scenes/rooms/room_1_02.tscn");
        let mut room02_pack = PackedScene::new();
        room02_pack = load("res://scenes/rooms/room_1_03.tscn");
        let mut room03_pack = PackedScene::new();
        room03_pack = load("res://scenes/rooms/room_1_01.tscn");

        let mut room_01 = room01_pack.instantiate_as::<Node2D>();
        let mut room_02 = room02_pack.instantiate_as::<Node2D>();
        let mut room_03 = room03_pack.instantiate_as::<Node2D>();

        room_01.set_transform(Transform2D::IDENTITY.translated_local(Vector2::new(0.0, 720.0)));
        room_02.set_transform(Transform2D::IDENTITY.translated_local(Vector2::new(0.0, 1440.0)));
        room_03.set_transform(Transform2D::IDENTITY.translated_local(Vector2::new(1280.0, 1440.0)));

        self.base.add_child(room_01.upcast());
        self.base.add_child(room_02.upcast());
        self.base.add_child(room_03.upcast());
        godot_warn!("{}", self.base.get_meta("floor".into()));
    }

    fn process(&mut self, _delta: f64) {
    }
}
