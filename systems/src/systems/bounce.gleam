import gleam/float.{compare, negate}
import gleam/order.{Gt, Lt}
import types/physics.{
  type Position, type Velocity, type WindowFrame, Position, Velocity,
}

pub fn do_bounce(
  pos: Position,
  vel: Velocity,
  size: Float,
  win: WindowFrame,
) -> #(Position, Velocity) {
  let half = size /. 2.0
  let new_x = pos.x +. vel.x
  let new_y = pos.y +. vel.y

  let bounced_vx = case compare(new_x +. half, win.width) {
    Gt -> negate(vel.x)
    _ ->
      case compare(new_x -. half, negate(win.width)) {
        Lt -> negate(vel.x)
        _ -> vel.x
      }
  }

  let bounced_vy = case compare(new_y +. half, win.height) {
    Gt -> negate(vel.y)
    _ ->
      case compare(new_y -. half, negate(win.height)) {
        Lt -> negate(vel.y)
        _ -> vel.y
      }
  }

  #(Position(new_x, new_y), Velocity(bounced_vx, bounced_vy))
}
