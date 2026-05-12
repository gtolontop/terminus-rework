extends Area2D

var inbox = false

func _process(_delta: float) -> void:
	if general.peut_attraper and inbox and Input.is_action_pressed("mvcatch"):
		general.fmvcatch("pilier3")
		self.queue_free()

	position = general.pilier3pos


func _on_body_exited(body: Node2D) -> void:
	if body.name == "player":
		inbox = false
		general.inboxpilier3 = false


func _on_body_entered(body: Node2D) -> void:
	if body.name == "player":
		inbox = true
		general.inboxpilier3 = true
