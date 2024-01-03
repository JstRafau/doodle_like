extends TextureButton


const RUN = preload("res://scenes/run.tscn")

func _pressed():
    get_tree().change_scene_to_packed(RUN)	
