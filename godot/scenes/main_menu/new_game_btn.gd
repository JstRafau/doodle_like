extends TextureButton


const ROOM = preload("res://scenes/rooms/example_room.tscn")


func _pressed():
    get_tree().change_scene_to_packed(ROOM)	
