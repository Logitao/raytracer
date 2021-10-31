use image::{ImageBuffer, Pixel, RgbImage};
use scene::Scene;
use vec3::Vec3;

use crate::{color::Color, light::Light, ray::Ray, sphere::Sphere};

mod camera;
mod color;
mod light;
mod ray;
mod scene;
mod sphere;
mod util;
mod vec3;

const IMAGE_WIDTH: u32 = 1920;
const IMAGE_HEIGHT: u32 = 1080;

fn main() {
    let scene = Scene::init(
        IMAGE_WIDTH,
        IMAGE_HEIGHT,
        vec![
            Sphere::new(
                250.0,
                Vec3::new(0.0, -200.0, 700.0),
                Color::new(0.0, 256.0, 0.0),
            ),
            Sphere::new(
                250.0,
                Vec3::new(0.0, 0.0, 700.0),
                Color::new(256.0, 0.0, 0.0),
            ),
            Sphere::new(
                50.0,
                Vec3::new(0.0, 0.0, 100.0),
                Color::new(0.0, 0.0, 256.0),
            ),
        ],
        vec![Light::new(Vec3::new(300.0, 300.0, 0.0), 10.0)],
    );

    for sphere in scene.spheres.iter() {
        println!("{}", (sphere.position - scene.camera.position).len());
    }

    let camera = &scene.camera;

    let mut img: RgbImage = ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let ray = Ray::make_for_pixel(&camera, x, y);
        let color = ray.trace(&scene);
        *pixel = image::Rgb([color.r as u8, color.g as u8, color.b as u8]);
    }

    img.save("test.png");
}
