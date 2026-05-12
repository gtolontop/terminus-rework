extends Node2D

var inrange = false

func _on_palourde_body_entered(body: Node2D) -> void:
	if body.name == "player":
		inrange = true

func _ready() -> void:
	general.current_scene = "depart"
	general.charger_scene()

func _on_palourde_body_exited(body: Node2D) -> void:
	if body.name == "player":
		inrange = false
		
func _process(_delta: float) -> void:
	if inrange and Input.is_action_pressed("cat"):
		general.playerpos = $player.position
		if "pwd" not in general.sorts:
			general.sorts.append("pwd")
		print(general.sorts)
		get_tree().change_scene_to_file("res://scenes/palourdekiparle.tscn")


func _on_toprairie_body_entered(body: Node2D) -> void:
	if body.name == "player":
		general.departtoprairie()


func _on_tobois_body_entered(body: Node2D) -> void:
	if body.name == "player":
		general.departtobois()
