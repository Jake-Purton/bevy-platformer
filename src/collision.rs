use bevy::{
    prelude::{Vec2, Vec3},
    sprite::collide_aabb::Collision,
};

// this function checks if there will be a collision after the velocities are added
// returns the collision and the depth
pub fn velocity_collision(
    a_pos: Vec3,
    a_size: Vec2,
    a_velocity: Vec2,
    b_pos: Vec3,
    b_size: Vec2,
    b_velocity: Vec2,
) -> Option<(Collision, f32)> {
    let a_min = a_pos.truncate() - a_size / 2.0;
    let a_max = a_pos.truncate() + a_size / 2.0;
    let b_min = b_pos.truncate() - b_size / 2.0;
    let b_max = b_pos.truncate() + b_size / 2.0;

    // min and max after velocity
    let a_new_min = (a_pos + Vec3::new(a_velocity.x, a_velocity.y, 0.0)).truncate() - a_size / 2.0;
    let a_new_max = (a_pos + Vec3::new(a_velocity.x, a_velocity.y, 0.0)).truncate() + a_size / 2.0;
    let b_new_min = (b_pos + Vec3::new(b_velocity.x, b_velocity.y, 0.0)).truncate() - b_size / 2.0;
    let b_new_max = (b_pos + Vec3::new(b_velocity.x, b_velocity.y, 0.0)).truncate() + b_size / 2.0;

    // check to see if the two rectangles are intersecting
    if a_new_min.x < b_new_max.x
        && a_new_max.x > b_new_min.x
        && a_new_min.y < b_new_max.y
        && a_new_max.y > b_new_min.y
    {
        if a_velocity.x - b_velocity.x < 0.0 
            && a_new_min.x < b_new_max.x 
            && a_min.x > b_max.x 
        {

            Some((Collision::Right, 0.0001 + b_new_max.x + (a_size.x / 2.0)))

        } else if a_velocity.x - b_velocity.x > 0.0
            && a_new_max.x > b_new_min.x
            && a_max.x < b_min.x
        {

            Some((Collision::Left, -0.0001 + b_new_min.x - (a_size.x / 2.0)))

        } else if a_velocity.y - b_velocity.y < 0.0
            && a_new_min.y < b_new_max.y
            && a_min.y > b_max.y
        {

            Some((Collision::Top, 0.0001 + b_new_max.y + (a_size.y / 2.0)))

        } else if a_velocity.y - b_velocity.y > 0.0
            && a_new_max.y > b_new_min.y
            && a_max.y < b_min.y
        {

            Some((Collision::Bottom, -0.0001 + b_new_min.y - (a_size.y / 2.0)))

        } else {

            Some((Collision::Inside, 0.0))

        }
    } else {
        None
    }
}
