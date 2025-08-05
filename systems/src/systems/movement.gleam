import types/physics.{type Position, type Velocity, Position}

pub fn do_movement(pos: Position, vel: Velocity, dt: Float) -> Position {
  let new_x = pos.x +. vel.x *. dt
  let new_y = pos.y +. vel.y *. dt
  Position(new_x, new_y)
}
