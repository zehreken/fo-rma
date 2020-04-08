use crate::camera::camera::*;
use crate::primitives::vec3::*;
use crate::ray::ray::*;
use crate::sphere::sphere::*;
use rand::Rng;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

#[derive(Clone)]
pub struct Scene {
    camera: Camera,
    objects: Vec<Sphere>,
    width: u32,
    height: u32,
    channel_count: usize,
    colors: Vec<Vec3>,
    pub pixels: Vec<u8>,
}

pub fn create_scene(width: u32, height: u32, channel_count: usize) -> Scene {
    let camera = Camera::get_camera(width, height);

    Scene {
        camera,
        objects: get_objects(),
        width,
        height,
        channel_count,
        colors: vec![Vec3::zero(); width as usize * height as usize],
        pixels: vec![0; width as usize * height as usize * channel_count],
    }
}

pub fn update(scene: &mut Scene, keys: u8) {
    // 0000ADWS
    let mut velocity = Vec3::zero();
    if keys & 0b1000 == 0b1000 {
        velocity = velocity + Vec3::new(-0.02, 0.0, 0.0);
    }
    if keys & 0b100 == 0b100 {
        velocity = velocity + Vec3::new(0.02, 0.0, 0.0);
    }
    if keys & 0b10 == 0b10 {
        velocity = velocity + Vec3::new(0.0, 0.0, -0.02);
    }
    if keys & 0b1 == 0b1 {
        velocity = velocity + Vec3::new(0.0, 0.0, 0.02);
    }
    scene.camera.translate(velocity);
    render(scene);
}

fn render(scene: &mut Scene) {
    let width = scene.width;
    let height = scene.height;
    let channel_count = scene.channel_count;
    let (tx, rx): (Sender<(u8, Vec<u8>)>, Receiver<(u8, Vec<u8>)>) = mpsc::channel();
    let mut children = Vec::new();
    const NTHREADS: u8 = 4;
    let t_height = height / NTHREADS as u32;
    let t_offset: f32 = 1.0 / NTHREADS as f32;

    for t in 0..NTHREADS {
        let thread_x = tx.clone();
        let mut scene_x = scene.clone();
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

pub fn save_image(width: u32, height: u32, sample: u32) {
    let mut img_buf = image::ImageBuffer::new(width, height);
    let mut rng = rand::thread_rng();
    let camera = Camera::get_camera(width, height);
    let objects = get_objects();

    for (x, y, pixel) in img_buf.enumerate_pixels_mut() {
        let mut col = Vec3::zero();
        for _z in 0..sample {
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

fn color(ray: Ray, objects: &[Sphere], depth: u8) -> Vec3 {
    let mut hit_record: HitRecord = HitRecord::new();
    let mut has_hit = false;
    let t_min: f32 = 0.0;
    let mut closest_so_far: f32 = std::f32::MAX;
    let mut temp_obj = Sphere::new(Vec3::zero(), 0.0, 0, Vec3::zero(), 0.0);

    for obj in objects {
        if obj.hit(ray, t_min, closest_so_far, &mut hit_record) {
            has_hit = true;
            closest_so_far = hit_record.t;
            temp_obj = *obj;
        }
    }

    if has_hit {
        let mut reflect_record: ReflectRecord =
            ReflectRecord::new(Ray::new(Vec3::zero(), Vec3::zero()), Vec3::zero());
        if depth < 50 && temp_obj.scatter(ray, &mut hit_record, &mut reflect_record) {
            return reflect_record.attenuation
                * color(reflect_record.scattered, objects, depth + 1);
        } else {
            return Vec3::zero();
        }
    } else {
        let unit_direction: Vec3 = ray.direction().unit_vector();
        let t: f32 = 0.5 * (unit_direction.y() + 1.0);

        return (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0);
    }
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
        Vec3::new(0.3, 0.1, 0.5),
        0.0,
    ));

    objects
}
