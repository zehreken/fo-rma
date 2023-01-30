use super::super::shapes::hitable::Hitable;
use super::primitives::vec3::*;
use super::ray::*;
use super::scene::Scene;
use super::scenes;
use rand::Rng;
use std::thread;

const CHANNEL_COUNT: usize = 3;
const MAX_DEPTH: u8 = 50;

pub struct TraceModel {
    pub scene: Scene,
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<u8>,
}

pub fn create_model(width: u32, height: u32) -> TraceModel {
    let resolution: usize = (width * height) as usize;

    TraceModel {
        scene: Scene::new(width, height),
        width,
        height,
        pixels: vec![0; resolution * CHANNEL_COUNT],
    }
}

pub fn update(model: &mut TraceModel, keys: u8, delta_time: f32) {
    // 00EQADWS
    let mut delta = Vec3::zero();
    if keys & 0b100_000 == 0b100_000 {
        delta = delta + Vec3::new(0.0, -1.0, 0.0) * delta_time;
    }
    if keys & 0b10_000 == 0b10_000 {
        delta = delta + Vec3::new(0.0, 1.0, 0.0) * delta_time;
    }
    if keys & 0b1000 == 0b1000 {
        delta = delta + Vec3::new(1.0, 0.0, 0.0) * delta_time;
    }
    if keys & 0b100 == 0b100 {
        delta = delta + Vec3::new(-1.0, 0.0, 0.0) * delta_time;
    }
    if keys & 0b10 == 0b10 {
        delta = delta + Vec3::new(0.0, 0.0, -1.0) * delta_time;
    }
    if keys & 0b1 == 0b1 {
        delta = delta + Vec3::new(0.0, 0.0, 1.0) * delta_time;
    }
    // scene.camera.translate(delta);
    model.scene.camera.orbit(delta);
    model.pixels = render_mt(model);
    // model.pixels = render(model);
}

fn render(model: &mut TraceModel) -> Vec<u8> {
    let width = model.width;
    let height = model.height;
    let mut rng = rand::thread_rng();
    let resolution: usize = (width * height) as usize;
    let mut pixels: Vec<u8> = vec![0; resolution * CHANNEL_COUNT];
    for y in 0..height {
        for x in 0..width {
            let index: usize = ((x + y * width as u32) * CHANNEL_COUNT as u32) as usize;
            let u: f32 = (x as f32 + rng.gen::<f32>()) / width as f32;
            let v: f32 = ((height - y) as f32 + rng.gen::<f32>()) / height as f32;
            let ray = model.scene.camera.get_ray(u, v);
            let color = get_color(ray, &model.scene.objects, 0);

            let r = color.r().sqrt(); // sqrt, gamma correction
            let g = color.g().sqrt();
            let b = color.b().sqrt();
            pixels[index] = (r * 255.0) as u8;
            pixels[index + 1] = (g * 255.0) as u8;
            pixels[index + 2] = (b * 255.0) as u8;
        }
    }

    pixels
}

fn render_mt(model: &TraceModel) -> Vec<u8> {
    let width = model.width;
    let height = model.height;
    const NTHREADS: u8 = 8;
    let t_height = height / NTHREADS as u32;
    let t_offset: f32 = 1.0 / NTHREADS as f32;
    let scene = &model.scene;
    let camera = model.scene.camera.clone();
    let t_resolution: usize = (width * t_height as u32) as usize;
    let mut results = Vec::with_capacity(NTHREADS as usize);

    for t_id in 0..NTHREADS {
        let result = thread::spawn(move || {
            let mut rng = rand::thread_rng();
            let mut pixels: Vec<u8> = vec![0; t_resolution * CHANNEL_COUNT];
            for y in 0..t_height {
                for x in 0..width {
                    let index: usize = ((x + y * width as u32) * CHANNEL_COUNT as u32) as usize;
                    let u: f32 = (x as f32 + rng.gen::<f32>()) / width as f32;
                    let mut v: f32 = ((t_height - y) as f32 + rng.gen::<f32>()) / height as f32; // invert y
                    v += t_id as f32 * t_offset;
                    let ray = camera.get_ray(u, v);
                    let color = get_color(ray, &scenes::get_simple_scene(), 0);

                    let r = color.r().sqrt();
                    let g = color.g().sqrt();
                    let b = color.b().sqrt();
                    pixels[index] = (r * 255.0) as u8;
                    pixels[index + 1] = (g * 255.0) as u8;
                    pixels[index + 2] = (b * 255.0) as u8;
                }
            }
            (t_id, pixels)
        });

        results.push(result);
    }

    let mut ids = Vec::with_capacity(NTHREADS as usize);
    for t_result in results {
        ids.push(t_result.join().unwrap());
    }

    // sort ids
    ids.sort_by(|a, b| b.0.cmp(&a.0));
    let mut sum = Vec::with_capacity(t_resolution * NTHREADS as usize * CHANNEL_COUNT);
    for mut id in ids {
        sum.append(&mut id.1);
    }

    sum
}

pub fn save_image_mt(model: &TraceModel, sample: u32) {
    let mut img_buf = image::ImageBuffer::new(model.width, model.height);

    let mut pixels_acc: Vec<f32> = vec![0.0; model.width as usize * model.height as usize * 3];
    for _ in 0..sample {
        let pixels = render_mt(model);
        for i in 0..pixels.len() {
            pixels_acc[i] += pixels[i] as f32 / sample as f32;
        }
    }

    let mut index = 0;
    for (_, _, pixel) in img_buf.enumerate_pixels_mut() {
        *pixel = image::Rgb([
            pixels_acc[index] as u8,
            pixels_acc[index + 1] as u8,
            pixels_acc[index + 2] as u8,
        ]);
        index += 3;
    }

    img_buf.save("out/basic_mt.png").unwrap();
}

pub fn save_image(model: &TraceModel, sample: u32) {
    let width = model.width;
    let height = model.height;
    let mut img_buf = image::ImageBuffer::new(width, height);
    let mut rng = rand::thread_rng();
    let camera = &model.scene.camera;
    let objects = &model.scene.objects;

    for (x, y, pixel) in img_buf.enumerate_pixels_mut() {
        let mut col = Vec3::zero();
        for _ in 0..sample {
            let u: f32 = (x as f32 + rng.gen::<f32>()) / width as f32;
            let v: f32 = ((height - y) as f32 + rng.gen::<f32>()) / height as f32; // invert y
            let ray = camera.get_ray(u, v);
            col = col + get_color(ray, &objects, 0);
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

fn get_color(ray: Ray, objects: &Vec<Box<dyn Hitable>>, depth: u8) -> Vec3 {
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
        if depth < MAX_DEPTH && obj.scatter(ray, &mut hit_record, &mut reflect_record) {
            return reflect_record.attenuation
                * get_color(reflect_record.scattered, objects, depth + 1);
        } else {
            return Vec3::zero();
        }
    } else {
        // No hit, assign sky color
        let unit_direction: Vec3 = ray.direction().unit_vector();
        let t: f32 = 0.5 * (unit_direction.y + 1.0);

        // This is the color of the sky
        return (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0);
    }
}
