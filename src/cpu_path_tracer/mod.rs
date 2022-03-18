pub mod aabb;
mod camera;
pub mod plane;
pub mod primitives;
mod ray;
pub mod sphere;
mod utility;
use camera::*;
use plane::*;
use primitives::vec3::*;
use rand::Rng;
use ray::*;
use sphere::*;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
pub mod hitable;
use hitable::*;

pub struct Scene {
    camera: Camera,
    objects: Vec<Box<dyn Hitable + Send>>,
    width: u32,
    height: u32,
    channel_count: usize, // remove, this does not belong to the scene
    colors: Vec<Vec3>,    // remove, this does not belong to the scene
    pub pixels: Vec<u8>,  // remove, this does not belong to the scene
}

pub fn create_scene(width: u32, height: u32, channel_count: usize) -> Scene {
    let camera = Camera::get_camera(width, height);
    let ray_count: usize = (width * height) as usize;

    Scene {
        camera,
        // objects: super::misc::strict_covers::get_objects(),
        // objects: get_simple_scene(),
        // objects: get_objects(),
        objects: get_plane_scene(),
        width,
        height,
        channel_count, // rgb
        colors: vec![Vec3::zero(); ray_count],
        pixels: vec![0; ray_count * channel_count],
    }
}

pub fn update(scene: &mut Scene, keys: u8, delta_time: f32) {
    // 0000ADWS
    let mut velocity = Vec3::zero();
    if keys & 0b1000 == 0b1000 {
        velocity = velocity + Vec3::new(-0.02, 0.0, 0.0) * delta_time;
    }
    if keys & 0b100 == 0b100 {
        velocity = velocity + Vec3::new(0.02, 0.0, 0.0) * delta_time;
    }
    if keys & 0b10 == 0b10 {
        velocity = velocity + Vec3::new(0.0, 0.0, -0.02) * delta_time;
    }
    if keys & 0b1 == 0b1 {
        velocity = velocity + Vec3::new(0.0, 0.0, 0.02) * delta_time;
    }
    scene.camera.translate(velocity);
    render(scene);
}

fn copy_scene(scene: &Scene) -> Scene {
    Scene {
        camera: scene.camera,
        objects: get_plane_scene(),
        width: scene.width,
        height: scene.height,
        channel_count: scene.channel_count,
        colors: scene.colors.clone(),
        pixels: scene.pixels.clone(),
    }
}

fn render(scene: &mut Scene) {
    let width = scene.width;
    let height = scene.height;
    let channel_count = scene.channel_count; // Color channel
    let (tx, rx): (Sender<(u8, Vec<u8>)>, Receiver<(u8, Vec<u8>)>) = mpsc::channel();
    let mut children = Vec::new();
    const NTHREADS: u8 = 6;
    let t_height = height / NTHREADS as u32;
    let t_offset: f32 = 1.0 / NTHREADS as f32;

    for t in 0..NTHREADS {
        let thread_x = tx.clone();
        let mut scene_x = copy_scene(&scene);
        let child = thread::spawn(move || {
            let mut rng = rand::thread_rng();
            let size: usize = (width * t_height as u32) as usize;
            let mut pixels: Vec<u8> = vec![0; size * channel_count];
            for y in 0..t_height {
                for x in 0..width {
                    let color_index = (x + y * width as u32) as usize;
                    let index: usize = ((x + y * width as u32) * channel_count as u32) as usize;
                    let u: f32 = (x as f32 + rng.gen::<f32>()) / width as f32;
                    let mut v: f32 = ((t_height - y) as f32 + rng.gen::<f32>()) / height as f32; // invert y
                    v += t as f32 * t_offset;
                    let ray = scene_x.camera.get_ray(u, v);
                    scene_x.colors[color_index] = color(ray, &scene_x.objects, 0);

                    let r = scene_x.colors[color_index].r().sqrt();
                    let g = scene_x.colors[color_index].g().sqrt();
                    let b = scene_x.colors[color_index].b().sqrt();
                    pixels[index] = (r * 255.0) as u8;
                    pixels[index + 1] = (g * 255.0) as u8;
                    pixels[index + 2] = (b * 255.0) as u8;
                }
            }
            thread_x.send((t, pixels)).unwrap();
        });

        children.push(child);
    }

    let mut ids = Vec::with_capacity(NTHREADS as usize);
    for _ in 0..NTHREADS {
        ids.push(rx.recv().unwrap());
    }

    for child in children {
        child.join().unwrap();
    }

    // sort ids
    ids.sort_by(|a, b| b.0.cmp(&a.0));
    let mut sum = Vec::new();
    for mut id in ids {
        sum.append(&mut id.1);
    }

    scene.pixels = sum;
}

pub fn save_image_mt(scene: &mut Scene, sample: u32) {
    let mut img_buf = image::ImageBuffer::new(scene.width, scene.height);

    let mut pixels: Vec<f32> = vec![0.0; scene.width as usize * scene.height as usize * 3];
    println!("{}, {}", scene.pixels.len(), pixels.len());
    for _ in 0..sample {
        render(scene);
        for i in 0..scene.pixels.len() {
            pixels[i] += scene.pixels[i] as f32 / sample as f32;
        }
    }

    let mut index = 0;
    for (_, _, pixel) in img_buf.enumerate_pixels_mut() {
        *pixel = image::Rgb([
            pixels[index] as u8,
            pixels[index + 1] as u8,
            pixels[index + 2] as u8,
        ]);
        index += 3;
    }

    img_buf.save("out/basic_mt.png").unwrap();
}

pub fn save_image(width: u32, height: u32, sample: u32) {
    let mut img_buf = image::ImageBuffer::new(width, height);
    let mut rng = rand::thread_rng();
    let camera = Camera::get_camera(width, height);
    let objects = get_plane_scene();

    for (x, y, pixel) in img_buf.enumerate_pixels_mut() {
        let mut col = Vec3::zero();
        for _ in 0..sample {
            let u: f32 = (x as f32 + rng.gen::<f32>()) / width as f32;
            let v: f32 = ((height - y) as f32 + rng.gen::<f32>()) / height as f32; // invert y
            let ray = camera.get_ray(u, v);
            col = col + color(ray, &objects, 0);
        }

        col = col / sample as f32;
        col = Vec3::new(col.r().sqrt(), col.g().sqrt(), col.b().sqrt()); // Gamma correction

        *pixel = image::Rgb([
            (col.r() * 255.0) as u8,
            (col.g() * 255.0) as u8,
            (col.b() * 255.0) as u8,
        ]);
    }
    img_buf.save("out/basic.png").unwrap();
}

fn color(ray: Ray, objects: &Vec<Box<dyn Hitable + Send>>, depth: u8) -> Vec3 {
    let mut hit_record: HitRecord = HitRecord::new();
    let t_min: f32 = 0.001;
    let mut closest_so_far: f32 = std::f32::MAX;
    let mut temp_obj = None;

    for obj in objects {
        if obj.hit(ray, t_min, closest_so_far, &mut hit_record) {
            closest_so_far = hit_record.t;
            temp_obj = Some(obj);
        }
    }

    if let Some(obj) = temp_obj {
        let mut reflect_record: ReflectRecord =
            ReflectRecord::new(Ray::new(Vec3::zero(), Vec3::zero()), Vec3::zero());
        if depth < 50 && obj.scatter(ray, &mut hit_record, &mut reflect_record) {
            return reflect_record.attenuation
                * color(reflect_record.scattered, objects, depth + 1);
        } else {
            return Vec3::zero();
        }
    } else {
        // No hit, assign sky color
        let unit_direction: Vec3 = ray.direction().unit_vector();
        let t: f32 = 0.5 * (unit_direction.y() + 1.0);

        // This is the color of the sky
        return (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0);
    }
}

fn get_simple_scene() -> Vec<Sphere> {
    let mut objects: Vec<Sphere> = Vec::new();
    objects.push(Sphere::new(
        Vec3::new(-0.5, 0.0, 0.0),
        0.5,
        0,
        Vec3::new(0.5, 0.5, 0.5),
        0.0,
    ));
    objects.push(Sphere::new(
        Vec3::new(0.5, 0.0, 0.0),
        0.5,
        0,
        Vec3::new(0.7, 0.7, 0.7),
        0.0,
    ));
    objects.push(Sphere::new(
        Vec3::new(0.0, -100.5, 0.0),
        100.0,
        0,
        Vec3::new(0.7, 0.1, 0.7),
        0.0,
    ));

    objects
}

fn get_plane_scene() -> Vec<Box<dyn Hitable + Send>> {
    let mut objects: Vec<Box<dyn Hitable + Send>> = vec![];
    objects.push(Box::new(Plane::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
        0,
        Vec3::new(0.0, 1.0, 0.0),
        0.,
    )));
    // objects.push(Box::new(Sphere::new(
    //     Vec3::new(0.0, -100.5, -1.0),
    //     100.0,
    //     1,
    //     Vec3::new(1.0, 0.0, 0.0),
    //     0.6,
    // )));
    // objects.push(Box::new(Sphere::new(
    //     Vec3::new(0.0, 0.0, -1.0),
    //     0.5,
    //     1,
    //     Vec3::new(0.1, 0.1, 0.7),
    //     0.0,
    // )));
    // objects.push(Box::new(Sphere::new(
    //     Vec3::new(1.0, 0.0, -1.0),
    //     0.5,
    //     0,
    //     Vec3::new(0.9, 0.9, 0.9),
    //     0.2,
    // )));
    // objects.push(Box::new(Sphere::new(
    //     Vec3::new(-1.0, -0.0, -1.0),
    //     0.5,
    //     2,
    //     Vec3::new(1.0, 1.0, 1.0),
    //     0.2,
    // )));

    objects
}

fn get_objects() -> Vec<Sphere> {
    let mut objects: Vec<Sphere> = Vec::new();
    objects.push(Sphere::new(
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
        0, // lambertian
        Vec3::new(0.5, 0.1, 0.1),
        0.0,
    ));
    objects.push(Sphere::new(
        Vec3::new(1.0, 0.0, -1.0),
        0.5,
        1, // metal
        Vec3::new(0.9, 0.9, 0.9),
        0.2,
    ));
    objects.push(Sphere::new(
        Vec3::new(1.0, 0.0, -3.0),
        0.5,
        1, // metal
        Vec3::new(1.0, 1.0, 1.0),
        1.0,
    ));
    objects.push(Sphere::new(
        Vec3::new(-1.0, -0.0, -1.0),
        0.5,
        2, // dielectric
        Vec3::new(0.1, 0.5, 0.1).sqrt().sqrt().sqrt(),
        0.2,
    ));
    objects.push(Sphere::new(
        Vec3::new(0.0, 0.0, 1.0),
        0.5,
        2,
        Vec3::new(0.5, 0.5, 0.3).sqrt().sqrt().sqrt(),
        0.2,
    ));
    objects.push(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        0,
        Vec3::new(0.1, 0.3, 0.9),
        0.0,
    ));

    objects
}
