use glam::Vec3;

use crate::ball_obj::BallObject;

//OBB
pub fn check_ball_square_collision(
    ball_pos: Vec3,
    ball_radius: f32,
    square_pos: Vec3,
    square_size: f32,
    square_rotation: f32,
) -> (bool, usize, Vec3) {
    let half_size = square_size / 2.0;

    let sin_r = square_rotation.sin();
    let cos_r = square_rotation.cos();

    let local_x = cos_r * (ball_pos.x - square_pos.x) + sin_r * (ball_pos.y - square_pos.y);
    let local_y = -sin_r * (ball_pos.x - square_pos.x) + cos_r * (ball_pos.y - square_pos.y);
    let local_ball = Vec3::new(local_x, local_y, 0.0);

    let closest_x = local_ball.x.clamp(-half_size, half_size);
    let closest_y = local_ball.y.clamp(-half_size, half_size);
    let diff = local_ball - Vec3::new(closest_x, closest_y, 0.0);
    let dist_sq = diff.length_squared();

    let mut side: usize = 0;

    if dist_sq < ball_radius * ball_radius {
        if diff.x.abs() > diff.y.abs() {
            side = if diff.x > 0. { 0 } else { 2 };
        } else {
            side = if diff.y > 0. { 1 } else { 3 };
        }

        let dist = dist_sq.sqrt();
        let penetration = ball_radius - dist;
        let normal = if dist != 0.0 { diff / dist } else { Vec3::X };

        let correction_local = normal * penetration;
        let new_local_ball = local_ball + correction_local;

        let world_x = cos_r * new_local_ball.x - sin_r * new_local_ball.y + square_pos.x;
        let world_y = sin_r * new_local_ball.x + cos_r * new_local_ball.y + square_pos.y;

        let new_ball_pos = Vec3::new(world_x, world_y, 0.0);
        return (true, side, new_ball_pos);
    }

    (false, side, ball_pos)
}

pub fn check_ball_ball_collision(ball1: &BallObject, ball2: &BallObject) -> Option<Vec3> {
    let delta = ball2.position - ball1.position;
    let distance = delta.length();
    let mind_dist = ball1.radius + ball2.radius;

    if distance < mind_dist && distance > 0.0 {
        Some(delta.normalize())
    } else {
        None
    }
}

pub struct WallCollision {
    pub left: bool,
    pub right: bool,
    pub top: bool,
    pub bottom: bool,
}

pub fn check_wall_collision(
    ball_pos: Vec3,
    ball_radius: f32,
    screen_width: f32,
    screen_height: f32,
) -> WallCollision {
    let mut collision = WallCollision {
        left: false,
        right: false,
        top: false,
        bottom: false,
    };

    if ball_pos.x - ball_radius < 0.0 {
        collision.left = true;
    }
    if ball_pos.x + ball_radius > screen_width {
        collision.right = true;
    }
    if ball_pos.y - ball_radius < 0.0 {
        collision.bottom = true;
    }
    if ball_pos.y + ball_radius > screen_height {
        collision.top = true;
    }

    collision
}
