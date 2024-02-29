extends Control


func _ready():
    var players_corpse = get_tree().get_first_node_in_group("player")
    if players_corpse:
        players_corpse.queue_free()
