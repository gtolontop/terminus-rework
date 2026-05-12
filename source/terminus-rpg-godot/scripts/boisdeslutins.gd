extends Node2D

var inbox = false

func _ready() -> void:
	general.current_scene= "boisdeslutins"
	general.charger_scene()
	
func _on_todepart_body_entered(body: Node2D) -> void:
	if body.name == "player":
		general.boistodepart()

func _on_toacademie_body_entered(body: Node2D) -> void:
	if body.name == "player":
		general.boistoacademie()


func _on_panneau_body_entered(body: Node2D) -> void:
	if body.name == "player":
		inbox = true


func _on_panneau_body_exited(body: Node2D) -> void:
	if body.name == "player":
		inbox = false
		
func _process(_delta: float) -> void:
	if inbox and Input.is_action_pressed("cat"):
		general.playerpos = $player.position
		get_tree().change_scene_to_file("res://scenes/lirepanneau.tscn")
