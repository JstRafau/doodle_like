extends TextureButton

const PLAYER = preload("player.tscn")


func _pressed():
	print("Epic adventure beginzz B))")

	get_tree().change_scene_to_packed(PLAYER)	
