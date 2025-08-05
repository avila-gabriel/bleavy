import gleam/float.{compare, negate}
import gleam/order.{Gt, Lt}

pub type Position {
  Position(x: Float, y: Float)
}

pub type Velocity {
  Velocity(x: Float, y: Float)
}

pub type WindowFrame {
  WindowFrame(width: Float, height: Float)
}

fn move_position(
  pos: Position,
  vel: Velocity,
  dt: Float,
) -> #(Position, Velocity) {
  let Position(px, py) = pos
  let Velocity(vx, vy) = vel
  #(Position(px +. vx *. dt, py +. vy *. dt), vel)
}

fn do_bounce(
  pos: Position,
  vel: Velocity,
  size: Float,
  win: WindowFrame,
) -> #(Position, Velocity) {
  let Position(x, y) = pos
  let Velocity(vx, vy) = vel
  let WindowFrame(win_w, win_h) = win

  let half = size /. 2.0
  let new_x = x +. vx
  let new_y = y +. vy

  let bounced_vx = case compare(new_x +. half, win_w) {
    Gt -> negate(vx)
    _ ->
      case compare(new_x -. half, negate(win_w)) {
        Lt -> negate(vx)
        _ -> vx
      }
  }

  let bounced_vy = case compare(new_y +. half, win_h) {
    Gt -> negate(vy)
    _ ->
      case compare(new_y -. half, negate(win_h)) {
        Lt -> negate(vy)
        _ -> vy
      }
  }

  #(Position(new_x, new_y), Velocity(bounced_vx, bounced_vy))
}

pub fn move(
  x: Float,
  y: Float,
  vx: Float,
  vy: Float,
  dt: Float,
) -> #(Float, Float, Float, Float) {
  let pos = Position(x, y)
  let vel = Velocity(vx, vy)

  let #(Position(nx, ny), Velocity(nvx, nvy)) = move_position(pos, vel, dt)
  #(nx, ny, nvx, nvy)
}

pub fn bounce(
  x: Float,
  y: Float,
  vx: Float,
  vy: Float,
  size: Float,
  win_w: Float,
  win_h: Float,
) -> #(Float, Float, Float, Float) {
  let pos = Position(x, y)
  let vel = Velocity(vx, vy)
  let win = WindowFrame(win_w, win_h)
  let #(Position(nx, ny), Velocity(nvx, nvy)) = do_bounce(pos, vel, size, win)

  #(nx, ny, nvx, nvy)
}
