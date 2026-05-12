extends Area2D
#professeur scenes code
var inbox = false


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(_delta: float) -> void:
	position = general.profpos
	if inbox and Input.is_action_pressed("cat") and not(general.mactive) :# and not(general.profattrape) :
		general.playerpos = get_parent().get_node("player").position
		if "mv" not in general.sorts:
			general.sorts.append("mv")
		print(general.sorts)
		get_tree().change_scene_to_file("res://scenes/professeurparle.tscn")
	if general.peut_attraper and Input.is_action_pressed("mvcatch") and inbox:
		general.fmvcatch("professeur")
		self.queue_free()
		
	
	

func _on_body_entered(body: Node2D) -> void:
	if body.name == "player":
		inbox = true
		general.inboxprof = true

func _on_body_exited(body: Node2D) -> void:
	if body.name == "player":
		inbox = false
		general.inboxprof = false
