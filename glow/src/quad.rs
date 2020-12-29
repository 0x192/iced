use crate::program;
use crate::Transformation;
use glow::HasContext;
use iced_graphics::layer;
use iced_native::Rectangle;

const MAX_INSTANCES: usize = 100_000;

#[derive(Debug)]
pub struct Pipeline {
    program: <glow::Context as HasContext>::Program,
    vertex_array: <glow::Context as HasContext>::VertexArray,
    instances: <glow::Context as HasContext>::Buffer,
    instance_buffer: Vec<Quad>,
    transform_location: <glow::Context as HasContext>::UniformLocation,
    scale_location: <glow::Context as HasContext>::UniformLocation,
    screen_height_location: <glow::Context as HasContext>::UniformLocation,
    current_transform: Transformation,
    current_scale: f32,
    current_target_height: u32,
}

/// modification on layer::quad for use with OpenGL (2 ES) without instancing
#[derive(Debug, Clone, Copy)]
#[repr(C)]
struct Quad {
    q_pos: [f32; 2],
    i_pos: [f32; 2],
    i_scale: [f32; 2],
    color: [f32; 4],
    border_color: [f32; 4],
    border_radius: f32,
    border_width: f32,
}

impl Quad {
    fn from(layer: &layer::Quad, q_pos: [f32; 2]) -> Quad {
        Self {
            q_pos,
            i_pos: layer.position,
            i_scale: layer.size,
            color: layer.color,
            border_color: layer.border_color,
            border_radius: layer.border_radius,
            border_width: layer.border_width,
        }
    }
}

#[allow(unsafe_code)]
unsafe impl bytemuck::Zeroable for Quad {}
#[allow(unsafe_code)]
unsafe impl bytemuck::Pod for Quad {}

const POSITIONS: [[f32;2];4] = [
    [0f32, 0f32],
    [0f32, 1f32],
    [1f32, 0f32],
    [1f32, 1f32],
];

fn instances_to_slice<'a>(instances: &[layer::Quad], buffer: &'a mut Vec<Quad>) -> &'a[u8] {
    buffer.clear();
    for instance in instances {
        for q_pos in &POSITIONS {
            buffer.push(Quad::from(instance, *q_pos));
        }
    }
    bytemuck::cast_slice(buffer)
}

impl Pipeline {
    pub fn new(gl: &glow::Context) -> Pipeline {
        let program = unsafe {
            program::create(
                gl,
                &[
                    (glow::VERTEX_SHADER, include_str!("shader/quad_GLSL100.vert")),
                    (glow::FRAGMENT_SHADER, include_str!("shader/quad_GLSL100.frag")),
                ],
            )
        };

        let transform_location =
            unsafe { gl.get_uniform_location(program, "u_Transform") }
                .expect("Get transform location");

        let scale_location =
            unsafe { gl.get_uniform_location(program, "u_Scale") }
                .expect("Get scale location");

        let screen_height_location =
            unsafe { gl.get_uniform_location(program, "u_ScreenHeight") }
                .expect("Get target height location");

        unsafe {
            gl.use_program(Some(program));

            let matrix: [f32; 16] = Transformation::identity().into();
            gl.uniform_matrix_4_f32_slice(
                Some(&transform_location),
                false,
                &matrix,
            );

            gl.uniform_1_f32(Some(&scale_location), 1.0);
            gl.uniform_1_f32(Some(&screen_height_location), 0.0);

            gl.use_program(None);
        }

        let (vertex_array, instances) =
            unsafe { create_instance_buffer(gl, MAX_INSTANCES) };

        Pipeline {
            program,
            vertex_array,
            instances,
            instance_buffer: Vec::with_capacity(MAX_INSTANCES*4),
            transform_location,
            scale_location,
            screen_height_location,
            current_transform: Transformation::identity(),
            current_scale: 1.0,
            current_target_height: 0,
        }
    }

    pub fn draw(
        &mut self,
        gl: &glow::Context,
        target_height: u32,
        instances: &[layer::Quad], //graphics/src/layer.rs::Quad
        transformation: Transformation,
        scale: f32,
        bounds: Rectangle<u32>,
    ) {
        unsafe {
            // gl.enable(glow::SCISSOR_TEST);
            // gl.scissor(
            //     bounds.x as i32,
            //     (target_height - (bounds.y + bounds.height)) as i32,
            //     bounds.width as i32,
            //     bounds.height as i32,
            // );

            gl.use_program(Some(self.program));
            gl.bind_vertex_array(Some(self.vertex_array));
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.instances));
        }

        if transformation != self.current_transform {
            unsafe {
                let matrix: [f32; 16] = transformation.into();
                gl.uniform_matrix_4_f32_slice(
                    Some(&self.transform_location),
                    false,
                    &matrix,
                );

                self.current_transform = transformation;
            }
        }

        if scale != self.current_scale {
            unsafe {
                gl.uniform_1_f32(Some(&self.scale_location), scale);
            }

            self.current_scale = scale;
        }

        if target_height != self.current_target_height {
            unsafe {
                gl.uniform_1_f32(
                    Some(&self.screen_height_location),
                    target_height as f32,
                );
            }

            self.current_target_height = target_height;
        }

        let mut i = 0;
        let total = instances.len();

        while i < total {
            let end = (i + MAX_INSTANCES).min(total);
            let amount = end - i;

            unsafe {
                gl.buffer_sub_data_u8_slice(
                    glow::ARRAY_BUFFER,
                    0,
                    instances_to_slice(&instances[i..end], &mut self.instance_buffer),
                );

                for i in 0..(amount as i32) {
                    gl.draw_arrays(
                        glow::TRIANGLE_STRIP,
                        i*4 as i32,
                        4,
                    );
                }
            }

            i += MAX_INSTANCES;
        }

        unsafe {
            gl.bind_vertex_array(None);
            gl.use_program(None);
            // gl.disable(glow::SCISSOR_TEST);
        }
    }
}

unsafe fn create_instance_buffer(
    gl: &glow::Context,
    size: usize,
) -> (
    <glow::Context as HasContext>::VertexArray,
    <glow::Context as HasContext>::Buffer,
) {
    let vertex_array = gl.create_vertex_array().expect("Create vertex array");
    let buffer = gl.create_buffer().expect("Create instance buffer");

    gl.bind_vertex_array(Some(vertex_array));
    gl.bind_buffer(glow::ARRAY_BUFFER, Some(buffer));
    gl.buffer_data_size(
        glow::ARRAY_BUFFER,
        (size * 4 * std::mem::size_of::<Quad>()) as i32,
        glow::DYNAMIC_DRAW,
    );

    let stride = std::mem::size_of::<layer::Quad>() as i32;

    gl.enable_vertex_attrib_array(0);
    gl.vertex_attrib_pointer_f32(0, 2, glow::FLOAT, false, stride, 0);
    gl.enable_vertex_attrib_array(1);
    gl.vertex_attrib_pointer_f32(0, 2, glow::FLOAT, false, stride, 2);
    gl.enable_vertex_attrib_array(2);
    gl.vertex_attrib_pointer_f32(1, 2, glow::FLOAT, false, stride, 2+2);
    gl.enable_vertex_attrib_array(3);
    gl.vertex_attrib_pointer_f32(2, 4, glow::FLOAT, false, stride, 2+2+2);
    gl.enable_vertex_attrib_array(4);
    gl.vertex_attrib_pointer_f32(
        3, // attribute
        4, // length/size
        glow::FLOAT,
        false,
        stride,
        2 + 2 + 2 + 4, // offset
    );
    gl.enable_vertex_attrib_array(5);
    gl.vertex_attrib_pointer_f32(
        4,
        1,
        glow::FLOAT,
        false,
        stride,
        2 + 2 + 2 + 4 + 4,
    );
    gl.enable_vertex_attrib_array(6);
    gl.vertex_attrib_pointer_f32(
        5,
        1,
        glow::FLOAT,
        false,
        stride,
        2 + 2 + 2 + 4 + 4 + 1,
    );

    gl.bind_vertex_array(None);
    gl.bind_buffer(glow::ARRAY_BUFFER, None);

    (vertex_array, buffer)
}
