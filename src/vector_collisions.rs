use bevy::prelude::*;

pub fn solve_intersection (

    a: Vec2, 
    b: Vec2, 
    c: Vec2, 
    d: Vec2, 

) -> Option<(Vec2, f32, f32)> {

    // 	computes the intersection between two line segments; a to b, and c to d

    let ab = b - a;
    let cd = d - c;

    // perp_dot = cross product
    let ab_cross_cd = ab.perp_dot(cd);

    if ab_cross_cd == 0.0 {

        return None;

    } else {

		let ac = c - a;
		let t1 = ac.perp_dot(cd) / ab_cross_cd;
		let t2 = -ab.perp_dot(ac) / ab_cross_cd;
        return Some((a + ab * t1, t1, t2));

    }

    // if t1 == t2 then we know that the two points are in the same place at the same time

}

// calculates collisions between two squares
pub fn vector_collision (
    a_pos: Vec3,
    a_size: Vec2,
    a_velocity: Vec2,
    b_pos: Vec3,
    b_size: Vec2,
    b_velocity: Vec2,
) {

    // The two squares have 4 edges each. For each edge, in one square check if the two points that define the edge
    // intersect with each point on every other edge

    let mut a_corners: Vec<Vec2> = Vec::new();
    let mut b_corners: Vec<Vec2> = Vec::new();
    

}