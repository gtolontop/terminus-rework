extends Node2D
#cours scenes code

func _ready() -> void:
	general.charger_scene()


func _process(_delta: float) -> void:
	pass
func _on_toacademie_body_entered(body: Node2D) -> void:
	if body.name == "player":
		general.courstoacademie()
