extends Node2D

var etape =0
func _ready() -> void:
	$bonomeparle.visible = false
	$demandenom/blcdetonom.visible = false
	$demandenom.visible = false
	$demandenom/labarre.visible = false
	$tonadresse.visible = false
	$tonadresse/labarre.visible = false
	$tonadresse/tabitoufdp.visible = false
	$tuto.visible = false
	
	
	etape+=1

func _process(_delta: float) -> void:
	if etape == 1:
		$bonomeparle.visible = true
	elif etape == 2:
		$bonomeparle.visible = false
		
		$demandenom.visible = true
		$demandenom/blcdetonom.visible = true
		$demandenom/labarre.visible = true

	elif etape == 3:
		$demandenom.visible = false
		$demandenom/blcdetonom.visible = false
		$demandenom/labarre.visible = false
		
		$tonadresse.visible = true
		$tonadresse/labarre.visible = true
		$tonadresse/tabitoufdp.visible = true
		
	elif etape == 4:
		$tonadresse.visible = false
		$tonadresse/labarre.visible = false
		$tonadresse/tabitoufdp.visible = false
		
		$tuto.visible = true
	elif etape == 5:
		get_tree().change_scene_to_file("res://scenes/depart.tscn")
	
	
	if Input.is_action_just_pressed("enter"):
		etape+=1
