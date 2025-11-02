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
