use nalgebra_glm::{Vec3, Mat4};
use minifb::{Key, Window, WindowOptions};
use std::time::Duration;
use std::f32::consts::PI;
use uniforms::Uniforms;

mod bounding_box;
mod color;
mod framebuffer;
mod fragments;
mod vertex;
mod obj;
mod shader;
mod triangle;
mod uniforms;
mod line;

use framebuffer::Framebuffer;
use vertex::Vertex;
use obj::Obj;
use triangle::triangle;
use shader::vertex_shader;


fn main() {
    // Window
    let window_width = 800;
    let window_height = 600;
    let mut window = Window::new(
        "3D modeling - Zyron Starship",
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .unwrap();
    window.set_position(500, 500);
    window.update();

    // Framebuffer
    let framebuffer_width = 800;
    let framebuffer_height = 600;
    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);
    let frame_delay = Duration::from_millis(16);

    // Obj
    let object =  Obj::load("assets/ZyronStarship.obj").expect("Failed to load obj");
    let vertex_array = object.get_vertex_array();
    let light_dir= Vec3::new(1.0, 3.0, -4.0);

    // View Variables
    let mut translation = Vec3::new(300.0, 200.0, 0.0);
    let mut rotation = Vec3::new(0.0, 0.0, 0.0);
    let mut scale = 100.0f32;

    // Main Window Loop:
    while window.is_open() {
        // Closing listener
        framebuffer.clear();
        if window.is_key_down(Key::Escape) {
            break;
        }

        // Input listener
        handle_input(&window, &mut translation, &mut rotation, &mut scale);
        let model_matrix = create_model_matrix(translation, scale, rotation);
        let uniforms= Uniforms { model_matrix, light_dir };
        
        // Rendering stage
        uniforms::render(&mut framebuffer, &uniforms, &vertex_array);
        
        window
            .update_with_buffer(
                &framebuffer.color_array_to_u32(),
                framebuffer_width,
                framebuffer_height,
            )
            .unwrap();
        std::thread::sleep(frame_delay);
    }
}

fn create_model_matrix(
    translation: Vec3,
    scale: f32,
    rotation: Vec3
)
->Mat4 {
    
    let (sin_x, cos_x) = rotation.x.sin_cos();
    let (sin_y, cos_y) = rotation.y.sin_cos();
    let (sin_z, cos_z) = rotation.z.sin_cos();
    
    let rotation_matrix_x = Mat4::new(
        1.0,  0.0,    0.0,   0.0,
        0.0,  cos_x, -sin_x, 0.0,
        0.0,  sin_x,  cos_x, 0.0,
        0.0,  0.0,    0.0,   1.0,
    );
    let rotation_matrix_y = Mat4::new(
        cos_y,  0.0,  sin_y, 0.0,
        0.0,    1.0,  0.0,   0.0,
        -sin_y, 0.0,  cos_y, 0.0,
        0.0,    0.0,  0.0,   1.0,
    );
    let rotation_matrix_z = Mat4::new(
        cos_z, -sin_z, 0.0, 0.0,
        sin_z,  cos_z, 0.0, 0.0,
        0.0,    0.0,  1.0, 0.0,
        0.0,    0.0,  0.0, 1.0,
    );

    let rotation_matrix = rotation_matrix_z * rotation_matrix_y * rotation_matrix_x;

    let transform_matrix = Mat4::new(
        scale, 0.0,   0.0,   translation.x,
        0.0,   scale, 0.0,   translation.y,
        0.0,   0.0,   scale, translation.z,
        0.0,   0.0,   0.0,   1.0,
    );

    transform_matrix * rotation_matrix
}

fn handle_input(window: &Window, translation: &mut Vec3, rotation: &mut Vec3, scale: &mut f32) {
    let move_speed = 10.0; // Velocidad de movimiento
    let rotation_speed = 0.05; // Velocidad de rotación
    let zoom_speed = 0.5; // Velocidad de zoom

    // Movimiento de cámara con flechas de dirección
    if window.is_key_down(Key::Left) {
        translation.x -= move_speed; // Mover cámara hacia la izquierda
    }
    if window.is_key_down(Key::Right) {
        translation.x += move_speed; // Mover cámara hacia la derecha
    }
    if window.is_key_down(Key::Up) {
        translation.y -= move_speed; // Mover cámara hacia arriba
    }
    if window.is_key_down(Key::Down) {
        translation.y += move_speed; // Mover cámara hacia abajo
    }

    // Control de rotación (opcional, se puede ajustar)
    if window.is_key_down(Key::A) {
        rotation.y += rotation_speed; // Rotar en el eje Y hacia la izquierda
    }
    if window.is_key_down(Key::D) {
        rotation.y -= rotation_speed; // Rotar en el eje Y hacia la derecha
    }
    if window.is_key_down(Key::W) {
        rotation.x += rotation_speed; // Rotar en el eje X hacia arriba
    }
    if window.is_key_down(Key::S) {
        rotation.x -= rotation_speed; // Rotar en el eje X hacia abajo
    }

    // Zoom
    if window.is_key_down(Key::Q) {
        *scale += zoom_speed;  // Acercar (aumentar escala)
    }
    if window.is_key_down(Key::E) {
        *scale -= zoom_speed;  // Alejar (disminuir escala)
    }
}

