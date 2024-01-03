extends TextureButton


# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.

func _pressed():
	print("Bye, hi\n Sigh, Hawaii\nWe've never meant to part,\nSublime, thy art")
	get_tree().quit()
