extends TextureButton

const BOB = preload("res://scenes/characters/bob.tscn")
#const JOE = preload("res://scenes/characters/joe.tscn")


func _pressed():
	get_tree().change_scene_to_packed(BOB)	
