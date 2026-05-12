extends CharacterBody2D
#player scenes code

const speed = 800
var current_direction = "none"




func _ready() -> void:
	position = general.playerpos
	$AnimatedSprite2D.play("idleside")
	$mprof.visible = false
	$mpilier.visible = false
	
		
func _physics_process(delta: float) -> void:
	player_mouvement(delta)
		
	if general.mactive and Input.is_action_pressed("mvrelease") and "mv" in general.sorts :
		if general.obj_catched == "professeur" :
			general.fmvrelease("professeur",position)
		if general.obj_catched == "pilier1":
			general.fmvrelease("pilier1",position)
		if general.obj_catched == "pilier2":
			general.fmvrelease("pilier2",position)
		if general.obj_catched == "pilier3":
			general.fmvrelease("pilier3",position)

	if general.obj_catched == null:
		$mprof.visible = false
		$mpilier.visible = false
	elif general.obj_catched == "professeur" and "mv" in general.sorts:
		$mprof.visible = true
	elif (general.obj_catched == "pilier1" or general.obj_catched == "pilier2" or general.obj_catched == "pilier3") and "mv" in general.sorts:
		$mpilier.visible = true



func player_mouvement(_delta):
	
	if Input.is_action_pressed("ui_up"):
		play_anim(1)
		velocity.y = -speed
		velocity.x = 0
		current_direction = "up"
	elif Input.is_action_pressed("ui_down"):
		play_anim(1)
		velocity.y = speed
		velocity.x =0
		current_direction = "down"
	elif Input.is_action_pressed("ui_right"):
		play_anim(1)
		velocity.y = 0
		velocity.x = speed
		current_direction="right"
	elif Input.is_action_pressed("ui_left"):
		play_anim(1)
		velocity.y = 0
		velocity.x = -speed
		current_direction ="left"
	else:
		play_anim(0)
		velocity.x= 0
		velocity.y = 0
	move_and_slide()


func play_anim(mouvement):
	var dir = current_direction
	var anim = $AnimatedSprite2D
	if dir == "right":
		anim.flip_h = false
		if mouvement ==1:
			anim.play("walkside")
		elif mouvement == 0:
			anim.play("idleside")
	elif dir == "left":
		anim.flip_h = true
		if mouvement ==1:
			anim.play("walkside")
		elif mouvement == 0:
			anim.play("idleside")
	elif dir == "down":
		anim.flip_h = true
		if mouvement ==1:
			anim.play("walkfront")
		elif mouvement == 0:
			anim.play("idlefront")
	elif dir == "up":
		anim.flip_h = true
		if mouvement ==1:
			anim.play("walkback")
		elif mouvement == 0:
			anim.play("idleback")
