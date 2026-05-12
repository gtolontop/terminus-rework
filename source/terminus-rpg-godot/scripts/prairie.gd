extends Node2D


func _on_todepart_body_entered(body: Node2D) -> void:
	if body.name == "player":
		general.prairietodepart()

func _ready() -> void:
	general.current_scene = "prairie"
	general.charger_scene()
