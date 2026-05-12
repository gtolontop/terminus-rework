extends Node2D

#general scenes code
var isfullscreen = 0
var cameralimit = [0,1150,0,2400]

var playerpos = Vector2(1330,560)
var profpos = Vector2(1400,500)
var pilier1pos= Vector2(600,380)
var pilier2pos= Vector2(400,380)
var pilier3pos= Vector2(200,380)

var mvliste = []
var boiteliste = []

var inboxprof = false
var inboxpilier1 = false
var inboxpilier2 = false
var inboxpilier3 = false

var sorts = ["cd","cat","ls"]

var mactive = false
var peut_attraper = true
var mettredansboite = false

var obj_catched = null

var current_scene = "depart"
var profdansscene = "cours"
var pilier1dansscene = "salledentrainement"
var pilier2dansscene = "salledentrainement"
var pilier3dansscene = "salledentrainement"


@onready var loadprof: PackedScene = preload("res://scenes/professeur.tscn")
@onready var loadpilier1: PackedScene = preload("res://scenes/pilier1.tscn")
@onready var loadpilier2: PackedScene = preload("res://scenes/pilier2.tscn")
@onready var loadpilier3: PackedScene = preload("res://scenes/pilier3.tscn")




func fmvcatch(obj):
	if "mv" in sorts:
		mactive = true
		if obj == "boite":
			obj_catched = boiteliste[-1]
			boiteliste.pop_at(-1)
		else:
			obj_catched = obj


func fmvrelease(obj,pos):
	var profscene = general.loadprof.instantiate()
	var pilier1scene = general.loadpilier1.instantiate()
	var pilier2scene = general.loadpilier2.instantiate()
	var pilier3scene = general.loadpilier3.instantiate()
	
	if mettredansboite:
		boiteliste.append(obj)
		mactive = false
		obj_catched = null
	else:
		if obj == "professeur":
			profscene.position = Vector2(pos.x +200, pos.y)
			profpos= profscene.position
			profdansscene = current_scene
			mactive= false
			obj_catched = null
			charger_scene()
		elif obj == "pilier1":
			pilier1scene.position = Vector2(pos.x +100, pos.y + 50)
			pilier1pos= pilier1scene.position
			pilier1dansscene = current_scene
			mactive= false
			obj_catched = null
			charger_scene()
		elif obj == "pilier2":
			pilier2scene.position = Vector2(pos.x +100, pos.y + 50)
			pilier2pos= pilier2scene.position
			pilier2dansscene = current_scene
			mactive= false
			obj_catched = null
			charger_scene()
		elif obj == "pilier3":
			pilier3scene.position = Vector2(pos.x +100, pos.y + 50)
			pilier3pos= pilier3scene.position
			pilier3dansscene = current_scene
			mactive= false
			obj_catched = null
			charger_scene()



func charger_scene():

	var profscene = general.loadprof.instantiate()
	var pilier1scene = general.loadpilier1.instantiate()
	var pilier2scene = general.loadpilier2.instantiate()
	var pilier3scene = general.loadpilier3.instantiate()
	
	print(pilier2scene.position)
	print(profdansscene,current_scene)
	if profdansscene == current_scene and "professeur" not in boiteliste:
		get_tree().current_scene.call_deferred("add_child",profscene)
		profscene.position = profpos
		profscene.visible = true
	else:
		profscene.visible = false
		
	if pilier1dansscene == current_scene and "pilier1" not in boiteliste:
		get_tree().current_scene.call_deferred("add_child",pilier1scene)
		pilier1scene.position = pilier1pos
		pilier1scene.visible = true
	else: 
		pilier1scene.visible = false
		
	if pilier2dansscene == current_scene and "pilier2" not in boiteliste:
		get_tree().current_scene.call_deferred("add_child",pilier2scene)
		pilier2scene.position = pilier2pos
		pilier2scene.visible = true
	else: 
		pilier2scene.visible = false
	print(pilier2scene.position)
	if pilier3dansscene == current_scene and "pilier2" not in boiteliste:
		get_tree().current_scene.call_deferred("add_child",pilier3scene)
		pilier3scene.position = pilier3pos
		pilier3scene.visible = true
	else: 
		pilier3scene.visible = false





func departtoprairie():
	current_scene = "prairie"
	get_tree().call_deferred("change_scene_to_file", "res://scenes/prairie.tscn")
	playerpos = Vector2(40,580)
	cameralimit =[0,1140,0,2430]
	current_scene ="prairie"
	charger_scene()
func prairietodepart():
	current_scene = "depart"
	get_tree().call_deferred("change_scene_to_file", "res://scenes/depart.tscn")
	playerpos = Vector2(2380,580)
	cameralimit =[0,1150,0,2400]
	current_scene ="depart"
	charger_scene()
func departtobois():
	current_scene= "boisdeslutins"
	get_tree().call_deferred("change_scene_to_file", "res://scenes/boisdeslutins.tscn")
	playerpos = Vector2(2130,1480)
	cameralimit =[-800,1900,0,2250]
	current_scene ="boisdeslutins"
	charger_scene()
func boistodepart():
	current_scene = "depart"
	get_tree().call_deferred("change_scene_to_file","res://scenes/depart.tscn")
	playerpos = Vector2(50,450)
	cameralimit =[0,1150,0,2400]
	current_scene ="depart"
	charger_scene()
func boistoacademie():
	current_scene = "academiedesbots"
	get_tree().call_deferred("change_scene_to_file","res://scenes/academiedesbots.tscn")
	playerpos = Vector2(890,1290)
	cameralimit =[0,1400,-380,2170]
	charger_scene()
func academietobois():
	current_scene = "boisdeslutins"
	get_tree().call_deferred("change_scene_to_file","res://scenes/boisdeslutins.tscn")
	playerpos = Vector2(960,330)
	cameralimit =[-800,1900,0,2250]
	charger_scene()
func academietocours():
	current_scene= "cours"
	get_tree().call_deferred("change_scene_to_file","res://scenes/cours.tscn")
	playerpos = Vector2(50,500)
	cameralimit = [0,1000,0,1920]
	charger_scene()
func courstoacademie():
	current_scene = "academiedesbots"
	get_tree().call_deferred("change_scene_to_file","res://scenes/academiedesbots.tscn")
	playerpos = Vector2(2100,390)
	cameralimit =[0,1400,-380,2170]
	charger_scene()
func academietoentrainement():
	current_scene = "salledentrainement"
	get_tree().call_deferred("change_scene_to_file","res://scenes/salledentrainement.tscn")
	playerpos = Vector2(1600,390)
	cameralimit = [0,770,0,1660]
	charger_scene()
func entrainementtoacademie():
	current_scene = "academiedesbots"
	get_tree().call_deferred("change_scene_to_file","res://scenes/academiedesbots.tscn")
	playerpos = Vector2(-480,380)
	cameralimit =[0,1400,-380,2170]
	charger_scene()



func _ready() -> void:
	var scene_path = get_tree().current_scene.scene_file_path
	
	DisplayServer.window_set_mode(DisplayServer.WINDOW_MODE_WINDOWED)
	if  "depart" in scene_path :
		playerpos = Vector2(1330,560)
		cameralimit =[0,1150,0,2400]
		current_scene ="depart"
	elif  "prairie" in scene_path :
		playerpos = Vector2(40,580)
		cameralimit =[0,1140,0,2430]
		current_scene ="prairie"
	elif "boisdeslutins" in scene_path  :
		playerpos = Vector2(2130,1480)
		cameralimit =[-800,1900,0,2250]
		current_scene ="boisdeslutins"
	elif "academiedesbots" in scene_path  :
		playerpos = Vector2(890,1290)
		cameralimit =[0,1400,-380,2170]
		current_scene ="academiedesbots"
	elif "cours" in scene_path  :
		playerpos = Vector2(50,500)
		cameralimit = [0,1000,0,1920]
		current_scene= "cours"
	elif "entrainement" in scene_path:
		playerpos = Vector2(1600,390)
		cameralimit = [0,770,0,1660]
		current_scene = "salledentrainement"
	charger_scene()

func _process(_delta: float) -> void:
	print(boiteliste, obj_catched)
	if mactive == false and "mv" in sorts and (inboxprof or inboxpilier1):
		peut_attraper = true
