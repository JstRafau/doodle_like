extends TextureButton


const SELECTOR = preload("res://scenes/gui/character_selector/character_selection.tscn")
func _pressed():
    get_tree().change_scene_to_packed(SELECTOR)
    
