use bevy::{
    prelude::{Vec2, Vec3},
    sprite::collide_aabb::Collision,
};

#[derive(Debug)]
pub struct VelocityCollision {
    pub collision: Collision,
    pub new_position: f32,
    pub depth: f32,
}

impl VelocityCollision {
    fn new(collision: Collision, new_position: f32, depth: f32) -> Self {
        Self {
            collision,
            new_position,
            depth,
        }
    }
}

// this function checks if there will be a collision after the velocities are added
// returns the collision and the depth
// pub fn velocity_collision(
//     a_pos: Vec3,
//     a_size: Vec2,
//     a_velocity: Vec2,
//     b_pos: Vec3,
//     b_size: Vec2,
//     b_velocity: Vec2,
// ) -> Option<VelocityCollision> {
//     let a_min = a_pos.truncate() - a_size / 2.0;
//     let a_max = a_pos.truncate() + a_size / 2.0;
//     let b_min = b_pos.truncate() - b_size / 2.0;
//     let b_max = b_pos.truncate() + b_size / 2.0;

//     // min and max after velocity
//     let a_new_min = (a_pos + Vec3::new(a_velocity.x, a_velocity.y, 0.0)).truncate() - a_size / 2.0;
//     let a_new_max = (a_pos + Vec3::new(a_velocity.x, a_velocity.y, 0.0)).truncate() + a_size / 2.0;
//     let b_new_min = (b_pos + Vec3::new(b_velocity.x, b_velocity.y, 0.0)).truncate() - b_size / 2.0;
//     let b_new_max = (b_pos + Vec3::new(b_velocity.x, b_velocity.y, 0.0)).truncate() + b_size / 2.0;

//     // check to see if the two rectangles are intersecting
//     if a_new_min.x < b_new_max.x
//         && a_new_max.x > b_new_min.x
//         && a_new_min.y < b_new_max.y
//         && a_new_max.y > b_new_min.y
//     {
//         if a_velocity.x - b_velocity.x < 0.0 && a_new_min.x < b_new_max.x && a_min.x > b_max.x {
//             Some(VelocityCollision::new(
//                 Collision::Right,
//                 0.0001 + b_new_max.x + (a_size.x / 2.0),
//                 b_new_max.x - a_new_min.x,
//             ))
//         } else if a_velocity.x - b_velocity.x > 0.0
//             && a_new_max.x > b_new_min.x
//             && a_max.x < b_min.x
//         {
//             Some(VelocityCollision::new(
//                 Collision::Left,
//                 -0.0001 + b_new_min.x - (a_size.x / 2.0),
//                 -b_new_min.x + a_new_max.x,
//             ))
//         } else if a_velocity.y - b_velocity.y < 0.0
//             && a_new_min.y < b_new_max.y
//             && a_min.y > b_max.y
//         {
//             Some(VelocityCollision::new(
//                 Collision::Top,
//                 0.0001 + b_new_max.y + (a_size.y / 2.0),
//                 b_new_max.y - a_new_min.y,
//             ))
//         } else if a_velocity.y - b_velocity.y > 0.0
//             && a_new_max.y > b_new_min.y
//             && a_max.y < b_min.y
//         {
//             Some(VelocityCollision::new(
//                 Collision::Bottom,
//                 -0.0001 + b_new_min.y - (a_size.y / 2.0),
//                 -b_new_min.y + a_new_max.y,
//             ))
//         } else {
//             Some(VelocityCollision::new(Collision::Inside, 0.0, 0.0))
//         }
//     } else {
//         None
//     }
// }

pub fn velocity_collision(
    a_pos: Vec3,
    a_size: Vec2,
    a_velocity: Vec2,
    b_pos: Vec3,
    b_size: Vec2,
    b_velocity: Vec2,
) -> Option<VelocityCollision> {
    let a_min = a_pos.truncate() - a_size / 2.0;
    let a_max = a_pos.truncate() + a_size / 2.0;
    let b_min = b_pos.truncate() - b_size / 2.0;
    let b_max = b_pos.truncate() + b_size / 2.0;

    // min and max after velocity
    let a_new_min = (a_pos + Vec3::new(a_velocity.x, a_velocity.y, 0.0)).truncate() - a_size / 2.0;
    let a_new_max = (a_pos + Vec3::new(a_velocity.x, a_velocity.y, 0.0)).truncate() + a_size / 2.0;
    let b_new_min = (b_pos + Vec3::new(b_velocity.x, b_velocity.y, 0.0)).truncate() - b_size / 2.0;
    let b_new_max = (b_pos + Vec3::new(b_velocity.x, b_velocity.y, 0.0)).truncate() + b_size / 2.0;

    // Edge collisions
    if a_velocity.x - b_velocity.x < 0.0 
        && a_new_min.x < b_new_max.x 
        && a_min.x > b_max.x 
        && ((a_min.y < b_max.y  && a_min.y > b_min.y ) 
        || (a_max.y > b_min.y  && a_max.y < b_max.y ))
        {
        Some(VelocityCollision::new(
            Collision::Right,
            0.0001 + b_new_max.x + (a_size.x / 2.0),
            b_new_max.x - a_new_min.x,
        ))
    } else if a_velocity.x - b_velocity.x > 0.0
        && a_new_max.x > b_new_min.x
        && a_max.x < b_min.x
        && ((a_min.y < b_max.y  && a_min.y > b_min.y ) 
        || (a_max.y > b_min.y  && a_max.y < b_max.y ))
    {
        Some(VelocityCollision::new(
            Collision::Left,
            -0.0001 + b_new_min.x - (a_size.x / 2.0),
            -b_new_min.x + a_new_max.x,
        ))
    } else if a_velocity.y - b_velocity.y < 0.0
        && a_new_min.y < b_new_max.y
        && a_min.y > b_max.y
        && ((a_min.x < b_max.x  && a_min.x > b_min.x ) 
        || (a_max.x > b_min.x  && a_max.x < b_max.x ))
    {
        Some(VelocityCollision::new(
            Collision::Top,
            0.0001 + b_new_max.y + (a_size.y / 2.0),
            b_new_max.y - a_new_min.y,
        ))
    } else if a_velocity.y - b_velocity.y > 0.0
        && a_new_max.y > b_new_min.y
        && a_max.y < b_min.y
        && ((a_min.x < b_max.x  && a_min.x > b_min.x ) 
        || (a_max.x > b_min.x  && a_max.x < b_max.x ))
    {
        Some(VelocityCollision::new(
            Collision::Bottom,
            -0.0001 + b_new_min.y - (a_size.y / 2.0),
            -b_new_min.y + a_new_max.y,
        ))

        // corner collisions
    } else if a_min.y >= b_max.y
        && a_max.x <= b_min.x
        && a_new_max.x > b_new_min.x
        && a_new_min.y < b_new_max.y
    {
        println!("here");

        // the velocity that is highest does not get a collision
        if a_velocity.x - b_velocity.x >= - a_velocity.y + b_velocity.y {
            Some(VelocityCollision::new(
                Collision::Top,
                0.0001 + b_new_max.y + (a_size.y / 2.0),
                b_new_max.y - a_new_min.y,
            ))
        } else {
            Some(VelocityCollision::new(
                Collision::Left,
                -0.0001 + b_new_min.x - (a_size.x / 2.0),
                -b_new_min.x + a_new_max.x,
            ))
        }
    
    } else {
        None
    }
}

