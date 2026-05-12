extends Node2D

func _ready() -> void:
	general.current_scene = "salledentrainement"
	general.charger_scene()


func _on_toacademie_body_entered(body: Node2D) -> void:
	if body.name == "player":
		general.entrainementtoacademie()
