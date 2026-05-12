extends Area2D

var inbox = false

# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(_delta: float) -> void:
	if inbox :
		general.mettredansboite = true
	else:
		general.mettredansboite = false
	if Input.is_action_pressed("mvcatch") and inbox and len(general.boiteliste) > 0 and general.peut_attraper:
		general.fmvcatch("boite")


func _on_body_entered(body: Node2D) -> void:
	if body.name == "player":
		inbox= true


func _on_body_exited(body: Node2D) -> void:
	if body.name == "player":
		inbox= false
