use bevy::{
    prelude::{Vec2, Vec3},
};

#[derive(Debug)]
pub struct VelocityCollision {
    pub collision: BetterCollision,
    pub new_position: f32,
    pub depth: f32,
}

impl VelocityCollision {
    fn new(collision:BetterCollision, new_position: f32, depth: f32) -> Self {
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
//                BetterCollision::Right,
//                 0.0001 + b_new_max.x + (a_size.x / 2.0),
//                 b_new_max.x - a_new_min.x,
//             ))
//         } else if a_velocity.x - b_velocity.x > 0.0
//             && a_new_max.x > b_new_min.x
//             && a_max.x < b_min.x
//         {
//             Some(VelocityCollision::new(
//                BetterCollision::Left,
//                 -0.0001 + b_new_min.x - (a_size.x / 2.0),
//                 -b_new_min.x + a_new_max.x,
//             ))
//         } else if a_velocity.y - b_velocity.y < 0.0
//             && a_new_min.y < b_new_max.y
//             && a_min.y > b_max.y
//         {
//             Some(VelocityCollision::new(
//                BetterCollision::Top,
//                 0.0001 + b_new_max.y + (a_size.y / 2.0),
//                 b_new_max.y - a_new_min.y,
//             ))
//         } else if a_velocity.y - b_velocity.y > 0.0
//             && a_new_max.y > b_new_min.y
//             && a_max.y < b_min.y
//         {
//             Some(VelocityCollision::new(
//                BetterCollision::Bottom,
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

//     // Edge collisions
//     if a_velocity.x - b_velocity.x < 0.0 
//         && a_new_min.x < b_new_max.x 
//         && a_min.x > b_max.x 
//         && ((a_min.y < b_max.y  && a_min.y > b_min.y ) 
//         || (a_max.y > b_min.y  && a_max.y < b_max.y ))
//         {
        // Some(VelocityCollision::new(
        //    BetterCollision::Right,
        //     0.0001 + b_new_max.x + (a_size.x / 2.0),
        //     b_new_max.x - a_new_min.x,
        // ))
//     } else if a_velocity.x - b_velocity.x > 0.0
//         && a_new_max.x > b_new_min.x
//         && a_max.x < b_min.x
//         && ((a_min.y < b_max.y  && a_min.y > b_min.y ) 
//         || (a_max.y > b_min.y  && a_max.y < b_max.y ))
//     {
//         Some(VelocityCollision::new(
//            BetterCollision::Left,
//             -0.0001 + b_new_min.x - (a_size.x / 2.0),
//             -b_new_min.x + a_new_max.x,
//         ))
//     } else if a_velocity.y - b_velocity.y < 0.0
//         && a_new_min.y < b_new_max.y
//         && a_min.y > b_max.y
//         && ((a_min.x < b_max.x  && a_min.x > b_min.x ) 
//         || (a_max.x > b_min.x  && a_max.x < b_max.x ))
//     {
        // Some(VelocityCollision::new(
        //    BetterCollision::Top,
        //     0.0001 + b_new_max.y + (a_size.y / 2.0),
        //     b_new_max.y - a_new_min.y,
        // ))
//     } else if a_velocity.y - b_velocity.y > 0.0
//         && a_new_max.y > b_new_min.y
//         && a_max.y < b_min.y
//         && ((a_min.x < b_max.x  && a_min.x > b_min.x ) 
//         || (a_max.x > b_min.x  && a_max.x < b_max.x ))
//     {
        // Some(VelocityCollision::new(
        //    BetterCollision::Bottom,
        //     -0.0001 + b_new_min.y - (a_size.y / 2.0),
        //     -b_new_min.y + a_new_max.y,
        // ))

//         // corner collisions
//     } else if a_min.y >= b_max.y
//         && a_max.x <= b_min.x
//         && a_new_max.x > b_new_min.x
//         && a_new_min.y < b_new_max.y
//     {
//         println!("here");

//         // the velocity that is highest does not get a collision
//         if a_velocity.x - b_velocity.x >= - a_velocity.y + b_velocity.y {
//             Some(VelocityCollision::new(
//                BetterCollision::Top,
//                 0.0001 + b_new_max.y + (a_size.y / 2.0),
//                 b_new_max.y - a_new_min.y,
//             ))
//         } else {
//             Some(VelocityCollision::new(
//                BetterCollision::Left,
//                 -0.0001 + b_new_min.x - (a_size.x / 2.0),
//                 -b_new_min.x + a_new_max.x,
//             ))
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
    let a_new_min = (a_pos.truncate() + a_velocity) - a_size / 2.0;
    let a_new_max = (a_pos.truncate() + a_velocity) + a_size / 2.0;
    let b_new_min = (b_pos.truncate() + b_velocity) - b_size / 2.0;
    let b_new_max = (b_pos.truncate() + b_velocity) + b_size / 2.0;

    let segment = find_segment(a_min, a_max, b_pos.truncate(), b_min, b_max);
    let new_segment = find_segment(a_new_min, a_new_max, b_pos.truncate() + b_velocity, b_new_min, b_new_max);

    // if the object has not moved
    if segment.x == new_segment.x && segment.y == new_segment.y {

        return None;

    // internal collision (ignored)
    } else if segment.x == XSegment::Middle && segment.y == YSegment::Middle {

        return None;

    // if x changed and not y
    } else if segment.x != new_segment.x && segment.y == segment.y {
        if segment.y == YSegment::Middle {
            if (a_velocity.x - b_velocity.x).is_sign_positive() {
                return Some(VelocityCollision::new(
                   BetterCollision::Left, 
                    -0.0001 + b_new_min.x - (a_size.x / 2.0), 
                    -b_new_min.x + a_new_max.x
                ));
            } else {
                return Some(VelocityCollision::new(
                   BetterCollision::Right,
                    0.0001 + b_new_max.x + (a_size.x / 2.0),
                    b_new_max.x - a_new_min.x,
                ));
            }
        } else {
            return None;
        }

    // if y changed but not x
    } else if segment.y != new_segment.y && segment.x == segment.x {
        if segment.x == XSegment::Middle {
            if (a_velocity.y - b_velocity.y).is_sign_positive() {
                return Some(VelocityCollision::new(
                   BetterCollision::Bottom,
                    -0.0001 + b_new_min.y - (a_size.y / 2.0),
                    -b_new_min.y + a_new_max.y,
                ));
            } else {
                return Some(VelocityCollision::new(
                   BetterCollision::Top,
                    0.0001 + b_new_max.y + (a_size.y / 2.0),
                    b_new_max.y - a_new_min.y,
                ));
            }
        } else {
            return None;
        }
    // if both have changed
    }  else if segment.y != new_segment.y && segment.x != new_segment.x {
        if segment.y == YSegment::Top {
            if segment.x == XSegment::Left {
                return Some(VelocityCollision::new(
                    BetterCollision::TopLeft,
                     0.0,
                     ((b_new_max.y - a_new_min.y).powi(2) + (b_new_min.x - a_new_max.x).powi(2)).sqrt(),
                 ));
            } else if segment.x == XSegment::Middle {
                return Some(VelocityCollision::new(
                    BetterCollision::Top,
                     0.0001 + b_new_max.y + (a_size.y / 2.0),
                     b_new_max.y - a_new_min.y,
                 ));
            } else {
                return Some(VelocityCollision::new(
                    BetterCollision::TopRight,
                     0.0,
                     ((b_new_max.y - a_new_min.y).powi(2) + (b_new_max.x - a_new_min.x).powi(2)).sqrt(),
                 ));
            }
        } else if segment.y == YSegment::Bottom {
            if segment.x == XSegment::Left {
                return Some(VelocityCollision::new(
                    BetterCollision::BottomLeft,
                     0.0,
                     ((b_new_min.y - a_new_max.y).powi(2) + (b_new_min.x - a_new_max.x).powi(2)).sqrt(),
                 ));
            } else if segment.x == XSegment::Middle {
                return Some(VelocityCollision::new(
                    BetterCollision::Bottom,
                     -0.0001 + b_new_min.y - (a_size.y / 2.0),
                     -b_new_min.y + a_new_max.y,
                 ));
            } else {
                return Some(VelocityCollision::new(
                    BetterCollision::BottomRight,
                     0.0,
                     ((b_new_min.y - a_new_max.y).powi(2) + (b_new_max.x - a_new_min.x).powi(2)).sqrt(),
                 ));
            }
        } else if segment.x == XSegment::Left {
            return Some(VelocityCollision::new(
                BetterCollision::Left, 
                 -0.0001 + b_new_min.x - (a_size.x / 2.0), 
                 -b_new_min.x + a_new_max.x
             ));
        } else {
            return Some(VelocityCollision::new(
                BetterCollision::Right,
                 0.0001 + b_new_max.x + (a_size.x / 2.0),
                 b_new_max.x - a_new_min.x,
             ));
        }

    } else {
        return None;
    }
}

#[derive(Debug, PartialEq)]
pub enum BetterCollision {
    Top,
    TopLeft,
    TopRight,
    Right,
    BottomRight,
    Bottom,
    BottomLeft,
    Left,
    // Inside,
}

#[derive(PartialEq)]
enum XSegment {
    Left,
    Middle,
    Right,
}

#[derive(PartialEq)]
enum YSegment {
    Top,
    Middle,
    Bottom,
}

struct Segment {
    x: XSegment,
    y: YSegment,
}

fn find_segment (
    a_min: Vec2,
    a_max: Vec2,
    center: Vec2,
    b_min: Vec2,
    b_max: Vec2,
) -> Segment {

    let closest_point = find_closest_point(a_min, a_max, center);

    if closest_point.y >= b_max.y {
        if closest_point.x <= b_min.x {
            return Segment { x: XSegment::Left, y: YSegment::Top};
        } else if closest_point.x >= b_max.x {
            return Segment { x: XSegment::Right, y: YSegment::Top};
        } else {
            return Segment { x: XSegment::Middle, y: YSegment::Top};
        }
    } else if closest_point.y <= b_min.y {
        if closest_point.x <= b_min.x {
            return Segment { x: XSegment::Left, y: YSegment::Bottom};
        } else if closest_point.x >= b_max.x {
            return Segment { x: XSegment::Right, y: YSegment::Bottom};
        } else {
            return Segment { x: XSegment::Middle, y: YSegment::Bottom};
        }
    } else {
        if closest_point.x <= b_min.x {
            return Segment { x: XSegment::Left, y: YSegment::Middle};
        } else if closest_point.x >= b_max.x {
            return Segment { x: XSegment::Right, y: YSegment::Middle};
        } else {
            return Segment { x: XSegment::Middle, y: YSegment::Middle};
        }
    }

}

fn find_closest_point (
    min: Vec2,
    max: Vec2,
    center: Vec2,
) -> Vec2 {

    let mut closest_point: Option<(Vec2, f32)> = None;

    let mut points = Vec::new();
    points.push(Vec2::new(min.x, max.y));
    points.push(Vec2::new(min.x, min.y));
    points.push(Vec2::new(max.x, min.y));
    points.push(Vec2::new(max.x, max.y));

    for point in points {

        let distance = ((point.x - center.x).powi(2) + (point.y - center.y).powi(2)).sqrt();

        if let Some(a) = closest_point {
            if a.1 > distance {
                closest_point = Some((point, distance));
            }
        } else {
            closest_point = Some((point, distance));
        }
    }

    closest_point.unwrap().0
}