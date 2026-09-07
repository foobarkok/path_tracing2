use glam::*;
use std::f32;

#[derive(Clone, Copy, Debug)]
pub struct Camera {
    pub image_width: u32,
    pub image_height: u32,
    pub center: Vec3A,
    pub pixel00_loc: Vec3A,
    pub pixel_delta_u: Vec3A,
    pub pixel_delta_v: Vec3A,
    pub focus_dist: f32,
    pub defocus_disk_u: Vec3A,
    pub defocus_disk_v: Vec3A,
}
impl Camera {
    pub fn new(
        aspect_ratio: f32,
        image_width: u32,
        vfov: f32,
        lookfrom: Vec3A,
        lookat: Vec3A,
        vup: Vec3A,
        defocus_angle: f32,
        focus_dist: f32,
    ) -> Self {
        let image_height: u32 = (image_width as f32 / aspect_ratio) as u32;
        let image_height: u32 = if image_height < 1 { 1 } else { image_height };

        let center = lookfrom;

        // Viewport
        let viewport_height: f32 =
            2.0 * focus_dist * ((vfov * (f32::consts::PI / 180.0)) / 2.0).tan();
        let viewport_width: f32 = viewport_height * (image_width as f32 / image_height as f32);

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        let w = (lookfrom - lookat).normalize();
        let u = vup.cross(w).normalize();
        let v = w.cross(u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = u * viewport_width;
        let viewport_v = -v * viewport_height;

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = viewport_u / image_width as f32;
        let pixel_delta_v = viewport_v / image_height as f32;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left = center - w * focus_dist - viewport_u * 0.5 - viewport_v * 0.5;
        let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        // Calculate the camera defocus disk basis vectors.
        let defocus_radius = focus_dist * ((defocus_angle / 2.0) * (f32::consts::PI / 180.0)).tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Self {
            image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            focus_dist,
            defocus_disk_u,
            defocus_disk_v,
        }
    }
}
