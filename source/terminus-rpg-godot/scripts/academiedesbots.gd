#academiedesbots scenes code

extends Node2D

func _ready() -> void:
	general.current_scene= "academiedesbots"
	general.charger_scene()
	


func _on_tobois_body_entered(body: Node2D) -> void:
	if body.name == "player":
		general.academietobois()


func _on_tocours_body_entered(body: Node2D) -> void:
	if body.name == "player":
		general.academietocours()
		
		
func _on_toentrainement_body_entered(body: Node2D) -> void:
	if body.name == "player":
		general.academietoentrainement()
