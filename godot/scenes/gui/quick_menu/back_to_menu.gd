extends Button


func _pressed():
    get_tree().paused = false
    get_tree().change_scene_to_file("res://scenes/gui/main_menu/main_menu.tscn")

