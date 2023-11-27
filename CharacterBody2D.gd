extends CharacterBody2D


@export var speed = 400
# Called every frame. 'delta' is the elapsed time since the previous frame.
func get_input(delta):
	var input_direction = Input.get_vector("mv_left", "mv_right", "mv_up", "mv_down")
	velocity = input_direction * speed * (60 * delta)

func _physics_process(delta):
	get_input(delta)
	move_and_slide()

