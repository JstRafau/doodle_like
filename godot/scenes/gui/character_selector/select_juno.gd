extends Button


const RUN = preload("res://scenes/run.tscn")
const JUNO = preload("res://scenes/characters/juno.tscn")

func _pressed():
    var juno_inst = JUNO.instantiate()
    
    get_window().add_child.call_deferred(juno_inst)
    get_tree().change_scene_to_packed(RUN)
