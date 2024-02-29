extends Button


const RUN = preload("res://scenes/run.tscn")
const BOB = preload("res://scenes/characters/bob.tscn")

func _pressed():
    var bob_inst = BOB.instantiate()
    
    get_window().add_child.call_deferred(bob_inst)
    get_tree().change_scene_to_packed(RUN)
