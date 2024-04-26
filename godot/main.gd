extends Control

var CENTER:Vector2=DisplayServer.window_get_size()/2
var R:float=100
var angle:float=0.0
var r:float=400
var da:float=TAU/5

var init_ball_number:int=8
var Ball:PackedScene=preload("res://ball.tscn")
var gravity:float=200

# Called when the node enters the scene tree for the first time.
func _ready():
	update_bhp()
	for i in init_ball_number:
		spawn_ball()

func _process(delta):
	update_info()

func update_info():
	%BallNumber.text=str(%Balls.get_child_count())
	%Frame.text=str(Engine.get_frames_per_second())

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _physics_process(delta):
	angle=angle+delta*da
	if angle>TAU:
		angle-=TAU
	update_bhp()
	update_bs()

func update_bhp():
	%BH1.position=CENTER+Vector2.from_angle(angle)*R
	%BH2.position=CENTER+Vector2.from_angle(angle-PI)*R

func update_bs():
	for ball:RigidBody2D in %Balls.get_children():
		var d1:Vector2=%BH1.position-ball.position
		var d2:Vector2=%BH2.position-ball.position
		#ball.apply_impulse(gravity*d1.normalized()/d1.length())
		#ball.apply_impulse(gravity*d2.normalized()/d2.length())
		ball.apply_impulse(gravity*d1.normalized()/d1.length()+gravity*d2.normalized()/d2.length())

func spawn_ball():
	var ball=Ball.instantiate()
	ball.position=CENTER+Vector2.from_angle(randf_range(0,TAU))*r
	%Balls.add_child(ball)

func eat_ball(ball:RigidBody2D):
	#%balls.remove_child(ball)
	ball.queue_free()
	spawn_ball()
	spawn_ball()

func _on_bh_body_entered(body):
	eat_ball(body)
