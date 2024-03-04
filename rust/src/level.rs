use godot::prelude::*;


#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct DDLLevel {
    #[base]
    base: Base<Node2D>,
}
