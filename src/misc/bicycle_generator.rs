use glam::{vec2, Vec2};
use rand::Rng;
use std::{f32::consts::PI, vec};

use crate::basics::scene_loader::{Object, Quat, Vec3};

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
    pub front_wheel_point: Circle,
    // back circle
    pub back_circle: Circle,
    pub back_wheel_point: Circle,
}

pub fn generate_bicycle() -> Bicycle {
    let pos = vec2(0.0, 20.0);
    let radius = 10.0;
    let main_circle = Circle::new(pos, radius);
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
        front_point: Circle::new(front_point, 1.0),
        back_point: Circle::new(back_point, 1.0),
        down_point: Circle::new(down_point, 1.0),
        front_circle,
        front_wheel_point: point_on_front,
        back_circle,
        back_wheel_point: point_on_back,
    }
}

pub fn generate_bicycle_objects() -> (Bicycle, Vec<Object>) {
    let bicycle = generate_bicycle();

    let mut objects = vec![];

    fn create_object(pos1: Vec2, pos2: Vec2) -> Object {
        let position = (pos1 + pos2) / 2.0;
        let direction = (pos2 - pos1).normalize();
        let half_angle = PI / 4.0 + direction.y.atan2(direction.x) / 2.0;
        let scale = (pos2 - pos1).length();
        let object = Object {
            mesh: "cylinder".to_owned(),
            material: "DiffuseColor".to_owned(),
            position: Vec3 {
                x: position.x,
                y: position.y,
                z: 0.0,
            },
            rotation: Quat {
                x: 0.0,
                y: 0.0,
                z: half_angle.sin(),
                w: half_angle.cos(),
            },
            scale: Vec3 {
                x: 1.0,
                y: scale,
                z: 1.0,
            },
        };
        object
    }
    fn create_wheel(position: Vec2) -> Object {
        let object = Object {
            mesh: "cylinder".to_owned(),
            material: "DiffuseColor".to_owned(),
            position: Vec3 {
                x: position.x,
                y: position.y,
                z: 0.0,
            },
            rotation: Quat {
                x: (PI / 4.0).sin(),
                y: 0.0,
                z: 0.0,
                w: (PI / 4.0).cos(),
            },
            scale: Vec3 {
                x: 20.0,
                y: 0.2,
                z: 20.0,
            },
        };
        object
    }

    let top_bar = create_object(bicycle.front_point.pos, bicycle.back_point.pos);
    objects.push(top_bar);

    let front_bar = create_object(bicycle.front_point.pos, bicycle.down_point.pos);
    objects.push(front_bar);

    let back_bar = create_object(bicycle.back_point.pos, bicycle.down_point.pos);
    objects.push(back_bar);

    let top_stay = create_object(bicycle.back_point.pos, bicycle.back_wheel_point.pos);
    objects.push(top_stay);
    let bottom_stay = create_object(bicycle.down_point.pos, bicycle.back_wheel_point.pos);
    objects.push(bottom_stay);
    let back_wheel = create_wheel(bicycle.back_wheel_point.pos);
    objects.push(back_wheel);

    let fork = create_object(bicycle.front_point.pos, bicycle.front_wheel_point.pos);
    objects.push(fork);
    let front_wheel = create_wheel(bicycle.front_wheel_point.pos);
    objects.push(front_wheel);

    return (bicycle, objects);
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
        let circle = Circle::new(midpoint, radius);
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
        return Some(Circle::new(circle_pos1, radius));
    } else {
        return Some(Circle::new(circle_pos2, radius));
    }
}

// Picks a random point on a circle constrained by the main_pos
fn find_point_on_circle(main_pos: Vec2, circle: &Circle) -> Circle {
    let direction = (circle.pos - main_pos).normalize();
    let point = Circle::new(circle.pos + direction * circle.r, 1.0);

    return point;
}

pub fn random_circle() -> Circle {
    let mut rng = rand::rng();
    Circle::new(
        vec2(rng.random_range(-10.0..10.0), rng.random_range(-10.0..10.0)),
        rng.random_range(20.0..30.0),
    )
}

fn random_pi() -> f32 {
    let mut rng = rand::rng();
    let limit = 0.3 * PI;
    rng.random_range(-limit..limit)
}

pub struct Circle {
    pub pos: Vec2,
    pub r: f32,
}

impl Circle {
    pub fn new(pos: Vec2, r: f32) -> Self {
        Self { pos, r }
    }
}
