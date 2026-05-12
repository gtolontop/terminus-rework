extends Area2D

var inbox = false

# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
# pilier1.gd - ne pAas écraser la position chaque frame
func _process(_delta: float) -> void:
	if general.peut_attraper and inbox and Input.is_action_pressed("mvcatch"):
		general.fmvcatch("pilier1")
		self.queue_free()
	position = general.pilier1pos


func _on_body_entered(body: Node2D) -> void:
	if body.name == "player":
		inbox = true
		general.inboxpilier1 = true


func _on_body_exited(body: Node2D) -> void:
	if body.name == "player":
		inbox = false
		general.inboxpilier1 = false
