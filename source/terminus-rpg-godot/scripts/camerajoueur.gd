extends Camera2D

var haut =0
var bas= 1
var gauche = 2
var droite = 3

# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	limit_top = general.cameralimit[0]
	limit_bottom = general.cameralimit[1]
	limit_left = general.cameralimit[2]
	limit_right =general.cameralimit[3]
	
# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(_delta: float) -> void:
	pass
