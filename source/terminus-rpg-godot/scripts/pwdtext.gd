extends RichTextLabel

func _ready() -> void:
	self.visible = false
	self.clear()


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(_delta: float) -> void:
	if Input.is_action_just_pressed("pwd") and "pwd" in general.sorts:
		if self.visible ==false:
			self.clear()
			self.add_text(general.current_scene.to_upper())
			self.visible =true
		else:
			self.visible =false
	
