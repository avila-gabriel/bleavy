import gleam/pair
import systems/bounce
import systems/movement
import types/physics.{Position, Velocity, WindowFrame}

pub fn movement(
  x: Float,
  y: Float,
  vx: Float,
  vy: Float,
  dt: Float,
) -> #(Float, Float) {
  // Construct domain objects
  let pos = Position(x, y)
  let vel = Velocity(vx, vy)

  // Invoke the pure‐logic system
  let new_pos = movement.do_movement(pos, vel, dt)

  // Destructure back to primitives
  let Position(nx, ny) = new_pos
  #(nx, ny)
}

pub fn bounce(
  pos: #(Float, Float),
  vel: #(Float, Float),
  size: Float,
  win_w: Float,
  win_h: Float,
) -> #(#(Float, Float), #(Float, Float)) {
  let pos = Position(pair.first(pos), pair.second(pos))
  let vel = Velocity(pair.first(vel), pair.second(vel))
  let win = WindowFrame(win_w, win_h)

  let #(Position(x, y), Velocity(vx, vy)) =
    bounce.do_bounce(pos, vel, size, win)

  #(#(x, y), #(vx, vy))
}
