extends Control


func _process(_delta):
    if Input.is_action_just_pressed("ui_cancel"):
        get_tree().paused = false
        queue_free()
