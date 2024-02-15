extends TileMap

func update_hp(hp, max_hp):
	
	for i in max_hp:
		if i < hp:
			set_cell(0, Vector2i(i, 0), 1, Vector2i(0, 0), 0)
			continue
		set_cell(0, Vector2i(i, 0), 1, Vector2i(0, 1), 0)

func _ready():
	var player: PlayerCharacter = get_tree().get_first_node_in_group("player")
	
	for i in player.get_meta("hp"):
		set_cell(0, Vector2i(i, 0), 1, Vector2i(0, 0), 0)
