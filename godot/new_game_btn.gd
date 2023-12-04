extends TextureButton

const PLAYER = preload("player.tscn")


func _pressed():
	get_tree().change_scene_to_packed(PLAYER)	
