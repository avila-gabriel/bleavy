import gleam/float.{compare, negate}
import gleam/order.{Gt, Lt}

pub fn move(
  x: Float,
  y: Float,
  vx: Float,
  vy: Float,
  dt: Float,
) -> #(Float, Float, Float, Float) {
  #(x +. vx *. dt, y +. vy *. dt, vx, vy)
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

  #(new_x, new_y, bounced_vx, bounced_vy)
}
