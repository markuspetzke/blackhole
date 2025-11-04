use glam::Vec3;

// AABB
pub fn check_ball_square_collision(
    mut ball_pos: Vec3,
    ball_radius: f32,
    square_pos: Vec3,
    square_size: f32,
) -> (bool, usize, Vec3) {
    let half_size = square_size / 2.0;

    let closest_x = ball_pos
        .x
        .clamp(square_pos.x - half_size, square_pos.x + half_size);
    let closest_y = ball_pos
        .y
        .clamp(square_pos.y - half_size, square_pos.y + half_size);

    let distance_x = ball_pos.x - closest_x;
    let distance_y = ball_pos.y - closest_y;

    let diff = Vec3::new(ball_pos.x - closest_x, ball_pos.y - closest_y, 0.0);
    let distance_squared = distance_x * distance_x + distance_y * distance_y;
    let mut side: usize = 0;

    if distance_squared < ball_radius * ball_radius {
        let dist = distance_squared.sqrt();
        let normal = if dist != 0.0 {
            diff / dist
        } else {
            Vec3::new(1.0, 0.0, 0.0)
        };

        let penetration = ball_radius - dist;

        let correction = normal * penetration;

        ball_pos += correction;

        if diff.x.abs() > diff.y.abs() {
            side = if diff.x > 0. { 0 } else { 2 };
        } else {
            side = if diff.y > 0. { 1 } else { 3 };
        }

        return (true, side, ball_pos);
    }

    (false, side, ball_pos)
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
