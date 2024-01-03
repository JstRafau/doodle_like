extends Button

const MENU = preload("res://scenes/gui/main_menu/main_menu.tscn")

func _pressed():
	print("pressed back")
	get_tree().paused = false
	get_tree().change_scene_to_packed(MENU)
