use glam::{vec2, Vec2};
use rand::Rng;
use std::f32::consts::PI;

/// I want you to do some research about the Vitruvian man
/// https://en.wikipedia.org/wiki/Vitruvian_Man
/// It was a search for the proportions of ideal man
/// A Vitruvian bike can be a similar search
pub struct Bicycle {
    // main circle
    pub main_circle: Circle,
    pub front_point: Circle,
    pub back_point: Circle,
    pub down_point: Circle,
    // front circle
    pub front_circle: Circle,
    pub point_on_front: Circle,
    // back circle
    pub back_circle: Circle,
    pub point_on_back: Circle,
}

pub fn generate_bicycle() -> Bicycle {
    let pos = vec2(0.0, 20.0);
    let radius = 10.0;
    let main_circle = Circle::new(pos.x, pos.y, radius);
    let front_point_angle = PI + random_pi();
    let front_point = (vec2(front_point_angle.cos(), front_point_angle.sin()) * radius) + pos;

    let back_point_angle = 0.0f32 + random_pi();
    let back_point = (vec2(back_point_angle.cos(), back_point_angle.sin()) * radius) + pos;

    let down_point_angle = 3.0 * PI / 2.0 + random_pi();
    let down_point = (vec2(down_point_angle.cos(), down_point_angle.sin()) * radius) + pos;

    let front_circle =
        find_circle_two_points_and_radius(front_point, down_point, radius, pos).unwrap();
    let back_circle =
        find_circle_two_points_and_radius(back_point, down_point, radius, pos).unwrap();

    let point_on_front = find_point_on_circle(pos, &front_circle);
    let point_on_back = find_point_on_circle(pos, &back_circle);

    Bicycle {
        main_circle,
        front_point: Circle::new(front_point.x, front_point.y, 1.0),
        back_point: Circle::new(back_point.x, back_point.y, 1.0),
        down_point: Circle::new(down_point.x, down_point.y, 1.0),
        front_circle,
        point_on_front,
        back_circle,
        point_on_back,
    }
}

fn find_circle_two_points_and_radius(
    p1: Vec2,
    p2: Vec2,
    radius: f32,
    main_pos: Vec2,
) -> Option<Circle> {
    let midpoint = (p1 + p2) / 2.0;
    let distance = (p2 - p1).length();
    if distance > 2.0 * radius {
        return None;
    }

    if distance == 2.0 * radius {
        let circle = Circle::new(midpoint.x, midpoint.y, radius);
        return Some(circle);
    }

    let h = (radius * radius - (distance / 2.0) * (distance / 2.0)).sqrt();
    let direction = (p2 - p1).normalize();
    let perpendicular = vec2(-direction.y, direction.x);

    let circle_pos1 = midpoint + perpendicular * h;
    let circle_pos2 = midpoint - perpendicular * h;

    let main_to_midpoint = (midpoint - main_pos).normalize();
    let dir1 = (circle_pos1 - main_pos).normalize();
    let dir2 = (circle_pos2 - main_pos).normalize();

    let dot1 = main_to_midpoint.dot(dir1);
    let dot2 = main_to_midpoint.dot(dir2);

    if dot1 > dot2 {
        return Some(Circle::new(circle_pos1.x, circle_pos1.y, radius));
    } else {
        return Some(Circle::new(circle_pos2.x, circle_pos2.y, radius));
    }
}

// Picks a random point on a circle constrained by the main_pos
fn find_point_on_circle(main_pos: Vec2, circle: &Circle) -> Circle {
    let direction = (vec2(circle.x, circle.y) - main_pos).normalize();
    let point = Circle::new(
        circle.x + direction.x * circle.r,
        circle.y + direction.y * circle.r,
        1.0,
    );

    return point;
}

pub fn random_circle() -> Circle {
    let mut rng = rand::rng();
    Circle::new(
        rng.random_range(-10.0..10.0),
        rng.random_range(-10.0..10.0),
        rng.random_range(20.0..30.0),
    )
}

fn random_pi() -> f32 {
    let mut rng = rand::rng();
    rng.random_range(-0.1 * PI..0.1 * PI)
}

pub struct Circle {
    pub x: f32,
    pub y: f32,
    pub r: f32,
}

impl Circle {
    pub fn new(x: f32, y: f32, r: f32) -> Self {
        Self { x, y, r }
    }
}
