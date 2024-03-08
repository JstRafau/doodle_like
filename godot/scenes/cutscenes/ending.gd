extends Node


func _ready():
    get_tree().get_first_node_in_group("player").queue_free()
    
func _on_timer_timeout() -> void:
    get_tree().change_scene_to_file("res://scenes/gui/main_menu/main_menu.tscn")
    
