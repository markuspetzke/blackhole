use glam::Vec3;

// AABB
pub fn check_ball_square_collision(
    ball_pos: Vec3,
    ball_radius: f32,
    square_pos: Vec3,
    square_size: f32,
) -> bool {
    let half_size = square_size / 2.0;

    let closest_x = ball_pos
        .x
        .clamp(square_pos.x - half_size, square_pos.x + half_size);
    let closest_y = ball_pos
        .y
        .clamp(square_pos.y - half_size, square_pos.y + half_size);

    let distance_x = ball_pos.x - closest_x;
    let distance_y = ball_pos.y - closest_y;
    let distance_squared = distance_x * distance_x + distance_y * distance_y;

    distance_squared < (ball_radius * ball_radius)
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
