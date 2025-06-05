//from Google Gemini,
//with Edits...
use pixels::{Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;
use glam::{Vec3, Mat4, Quat}; // For 3D math

// --- Constants ---
const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;

// --- Ray and HitRecord Structs ---

/// Represents a ray in 3D space.
#[derive(Debug, Clone, Copy)]
struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    fn new(origin: Vec3, direction: Vec3) -> Self {
        Ray {
            origin,
            direction: direction.normalize(), // Ensure direction is normalized
        }
    }

    /// Returns the point on the ray at a given distance `t`.
    fn at(&self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }
}

/// Stores information about a ray-object intersection.
#[derive(Debug, Clone, Copy)]
struct HitRecord {
    t: f32,       // Distance from ray origin to intersection point
    point: Vec3,  // World coordinates of intersection point
    normal: Vec3, // Surface normal at intersection point
    color: [u8; 4], // Color of the object
}

// --- Scene Objects (for this example, just Spheres) ---

struct Sphere {
    center: Vec3,
    radius: f32,
    color: [u8; 4],
}

impl Sphere {
    fn new(center: Vec3, radius: f32, color: [u8; 4]) -> Self {
        Sphere { center, radius, color }
    }

    /// Calculates ray-sphere intersection using the quadratic formula.
    /// Returns `Some(t)` if hit, otherwise `None`.
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            // No real solutions, ray misses the sphere
            None
        } else {
            // Find the two roots
            let sqrt_d = discriminant.sqrt();
            let mut t = (-b - sqrt_d) / (2.0 * a);

            if t < t_min || t > t_max {
                t = (-b + sqrt_d) / (2.0 * a);
                if t < t_min || t > t_max {
                    return None; // Both roots are out of range
                }
            }

            let point = ray.at(t);
            let normal = (point - self.center) / self.radius; // Unit normal vector
            Some(HitRecord {
                t,
                point,
                normal,
                color: self.color,
            })
        }
    }
}

// --- Camera ---

struct Camera {
    origin: Vec3,
    look_at: Vec3,
    up: Vec3,
    aspect_ratio: f32,
    vertical_fov: f32, // Field of View in degrees
    transform: Mat4,   // Camera to World transform
}

impl Camera {
    fn new(origin: Vec3, look_at: Vec3, up: Vec3, aspect_ratio: f32, vertical_fov: f32) -> Self {
        let mut cam = Camera {
            origin,
            look_at,
            up,
            aspect_ratio,
            vertical_fov,
            transform: Mat4::IDENTITY, // Will be calculated
        };
        cam.update_transform();
        cam
    }

    fn update_transform(&mut self) {
        // Simple look_at matrix creation (view matrix inverse)
        self.transform = Mat4::look_at_rh(self.origin, self.look_at, self.up).inverse();
    }

    /// Generates a ray for a given pixel coordinate (u, v)
    /// u, v are normalized from 0.0 to 1.0 (from top-left to bottom-right of viewport)
    fn get_ray(&self, u: f32, v: f32) -> Ray {
        // Calculate viewport dimensions in world space
        let theta = self.vertical_fov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = self.aspect_ratio * viewport_height;

        // Arbitrarily place the viewport 1 unit away from camera origin in its forward direction
        let w = (self.origin - self.look_at).normalize(); // Camera's forward (-Z axis in camera space)
        let u_vec = self.up.cross(w).normalize();           // Camera's right (X axis in camera space)
        let v_vec = w.cross(u_vec);                         // Camera's up (Y axis in camera space)

        let horizontal = u_vec * viewport_width;
        let vertical = v_vec * viewport_height;

        // Lower-left corner of the viewport
        let lower_left_corner = self.origin - horizontal / 2.0 - vertical / 2.0 - w;

        // Calculate ray direction based on pixel coordinates
        let direction_world = (lower_left_corner + u * horizontal + v * vertical) - self.origin;

        Ray::new(self.origin, direction_world)
    }
}


// --- Scene definition (list of objects) ---
struct Scene {
    spheres: Vec<Sphere>,
    ambient_light_color: Vec3,
    light_direction: Vec3, // Directional light for simplicity
}

impl Scene {
    fn new() -> Self {
        let mut spheres = Vec::new();
        spheres.push(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, [255, 0, 0, 255])); // Red sphere
        spheres.push(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, [100, 100, 100, 255])); // Large grey ground sphere
        spheres.push(Sphere::new(Vec3::new(1.0, 0.0, -1.5), 0.3, [0, 255, 0, 255])); // Green sphere
        spheres.push(Sphere::new(Vec3::new(-0.8, 0.2, -0.7), 0.4, [0, 0, 255, 255])); // Blue sphere

        Scene {
            spheres,
            ambient_light_color: Vec3::new(0.1, 0.1, 0.1), // Dim ambient light
            light_direction: Vec3::new(0.5, -1.0, -0.5).normalize(), // Light coming from top-right-front
        }
    }

    /// Finds the closest intersection of a ray with any object in the scene.
    fn hit_scene(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut hit_record: Option<HitRecord> = None;

        for sphere in &self.spheres {
            if let Some(hit) = sphere.hit(ray, t_min, closest_so_far) {
                closest_so_far = hit.t;
                hit_record = Some(hit);
            }
        }
        hit_record
    }

    /// Calculates the color for a given hit point.
    fn get_color(&self, hit: &HitRecord) -> [u8; 4] {
        let object_color = Vec3::new(
            hit.color[0] as f32 / 255.0,
            hit.color[1] as f32 / 255.0,
            hit.color[2] as f32 / 255.0,
        );

        // Simple diffuse lighting
        let normal = hit.normal;
        let light_dir = -self.light_direction; // Light comes FROM this direction

        let diffuse_intensity = normal.dot(light_dir).max(0.0);
        let diffuse_color = object_color * diffuse_intensity * Vec3::new(1.0, 1.0, 1.0); // White light

        // Combine ambient and diffuse
        let final_color_vec = self.ambient_light_color + diffuse_color;

        // Clamp to 0-1 and convert to u8
        [
            (final_color_vec.x.clamp(0.0, 1.0) * 255.0) as u8,
            (final_color_vec.y.clamp(0.0, 1.0) * 255.0) as u8,
            (final_color_vec.z.clamp(0.0, 1.0) * 255.0) as u8,
            hit.color[3], // Preserve original alpha
        ]
    }
}


// --- Main Ray Tracing Function ---

fn render(pixels_frame: &mut [u8], camera: &Camera, scene: &Scene) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            // Map pixel (x, y) to normalized (u, v) coordinates [0, 1]
            // We flip y for rendering, so (0,0) is bottom-left, (WIDTH, HEIGHT) is top-right
            let u = x as f32 / (WIDTH - 1) as f32;
            let v = (HEIGHT - 1 - y) as f32 / (HEIGHT - 1) as f32; // Invert Y for typical image coords

            let ray = camera.get_ray(u, v);

            let pixel_color: [u8; 4] = if let Some(hit) = scene.hit_scene(&ray, 0.001, f32::MAX) {
                // If the ray hit something, get its color
                scene.get_color(&hit)
            } else {
                // Background color (sky gradient)
                let t = 0.5 * (ray.direction.normalize().y + 1.0);
                let color = Vec3::new(1.0, 1.0, 1.0).lerp(Vec3::new(0.5, 0.7, 1.0), t); // Lerp white to blue
                [
                    (color.x * 255.0) as u8,
                    (color.y * 255.0) as u8,
                    (color.z * 255.0) as u8,
                    255,
                ]
            };

            // Write color to pixel buffer (RGBA)
            let idx = (y * WIDTH + x) as usize * 4;
            pixels_frame[idx..idx + 4].copy_from_slice(&pixel_color);
        }
    }
}

// --- Main Application Loop (using pixels and winit) ---
fn main() -> Result<(), pixels::Error> {
    let event_loop = EventLoop::new().unwrap();
    let mut input = WinitInputHelper::new();

    let window_size = LogicalSize::new(WIDTH, HEIGHT);
    let window = WindowBuilder::new()
        .with_title("Simple Ray Tracer in Rust")
        .with_inner_size(window_size)
        .with_min_inner_size(window_size)
        .build(&event_loop)
        .unwrap();

    let surface_texture = SurfaceTexture::new(WIDTH, HEIGHT, &window);
    let mut pixels = Pixels::new(WIDTH, HEIGHT, surface_texture)?;

    // Setup camera and scene
    let mut camera = Camera::new(
        Vec3::new(0.0, 0.0, 2.0), // Camera origin
        Vec3::new(0.0, 0.0, -1.0), // Look at -1.0 on Z axis
        Vec3::new(0.0, 1.0, 0.0), // Up direction
        WIDTH as f32 / HEIGHT as f32,
        90.0, // 90 degrees FOV
    );

    let scene = Scene::new();

    event_loop.run(move |event, elwt| {
        // Draw the current frame
        if let Event::AboutToWait = event {
            // In a real interactive raytracer, you'd update camera/scene state here
            // For this static example, we just re-render
            render(pixels.frame_mut(), &camera, &scene);

            // Present the frame
            if let Err(err) = pixels.render() {
                elwt.exit();
                eprintln!("pixels.render() failed: {:?}", err);
                return;
            }
            window.request_redraw(); // Request a redraw for the next frame
        }

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.close_requested() || input.key_pressed(winit::keyboard::KeyCode::Escape) {
                elwt.exit();
                return;
            }

            // Simple camera movement (for demonstration)
            let move_speed = 0.1;
            let rotate_speed = 0.05;

            // Movement
            if input.key_held(winit::keyboard::KeyCode::KeyW) { // Forward
                let forward = (camera.look_at - camera.origin).normalize();
                camera.origin += forward * move_speed;
                camera.look_at += forward * move_speed;
                camera.update_transform();
            }
            if input.key_held(winit::keyboard::KeyCode::KeyS) { // Backward
                let forward = (camera.look_at - camera.origin).normalize();
                camera.origin -= forward * move_speed;
                camera.look_at -= forward * move_speed;
                camera.update_transform();
            }
            if input.key_held(winit::keyboard::KeyCode::KeyA) { // Strafe Left
                let forward = (camera.look_at - camera.origin).normalize();
                let right = camera.up.cross(forward).normalize();
                camera.origin -= right * move_speed;
                camera.look_at -= right * move_speed;
                camera.update_transform();
            }
            if input.key_held(winit::keyboard::KeyCode::KeyD) { // Strafe Right
                let forward = (camera.look_at - camera.origin).normalize();
                let right = camera.up.cross(forward).normalize();
                camera.origin += right * move_speed;
                camera.look_at += right * move_speed;
                camera.update_transform();
            }

            // Rotation (Yaw)
            if input.key_held(winit::keyboard::KeyCode::ArrowLeft) {
                let current_dir = (camera.look_at - camera.origin).normalize();
                let rotation_quat = Quat::from_axis_angle(camera.up, rotate_speed); // Rotate around Y-axis (Up)
                let new_dir = rotation_quat * current_dir;
                camera.look_at = camera.origin + new_dir;
                camera.update_transform();
            }
            if input.key_held(winit::keyboard::KeyCode::ArrowRight) {
                let current_dir = (camera.look_at - camera.origin).normalize();
                let rotation_quat = Quat::from_axis_angle(camera.up, -rotate_speed);
                let new_dir = rotation_quat * current_dir;
                camera.look_at = camera.origin + new_dir;
                camera.update_transform();
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                if let Err(err) = pixels.resize_surface(size.width, size.height) {
                    elwt.exit();
                    eprintln!("pixels.resize_surface() failed: {:?}", err);
                    return;
                }
            }
        }
    }).unwrap();
}