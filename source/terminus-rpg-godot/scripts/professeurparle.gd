extends Node2D

#dialogue professeur scenes code

func _process(_delta: float) -> void:
	if Input.is_action_pressed("enter"):
		get_tree().change_scene_to_file("res://scenes/"+general.current_scene+".tscn")
