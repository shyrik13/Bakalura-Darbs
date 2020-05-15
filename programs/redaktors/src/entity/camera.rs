use crate::cgmath::*;

#[derive(Clone)]
pub struct Camera {
    pub position: Vector3<f32>,
    pub direction: [f32; 3],
    pub up: [f32; 3],
    pub view_matrix: Matrix4<f32>,
    pub projection_matrix: Matrix4<f32>,
    pub yaw: f32,
    pub pitch: f32,
    pub roll: f32,
    pub distance_from_followed_entity: f32,
    pub angle_around_followed_entity: f32,
    pub can_rotate: bool,
    pub last_mouse_position: [f32; 2],
    pub choosen_object_num: u32,
    pub mouse_ray: [f32; 3]
}

const SENSITIVITY: f32 = 0.1;

impl Camera {

    pub fn new(position: [f32; 3], direction: [f32; 3], up: [f32; 3]) -> Camera
    {
        let mut camera = Camera {
            position: Vector3 {
                x: position[0],
                y: position[1],
                z: position[2]
            },
            direction: direction,
            up: up,
            view_matrix: Matrix4::from_scale(0.0),
            projection_matrix: Matrix4::from_scale(0.0),
            yaw: 1.0,
            pitch: 1.0,
            roll: 0.0,
            distance_from_followed_entity: 0.0,
            angle_around_followed_entity: 0.0,
            can_rotate: false,
            last_mouse_position: [0.0, 0.0],
            choosen_object_num: 0,
            mouse_ray: [0.0, 0.0, 0.0]
        };
        camera.calculate_view_matrix();
        camera
    }

    pub fn calculate_view_matrix(&mut self)
    {

        let eye = cgmath::Point3 {
            x: self.position.x,
            y: self.position.y,
            z: self.position.z
        };

        let direction = cgmath::Vector3 {
            x: self.direction[0],
            y: self.direction[1],
            z: self.direction[2]
        };

        let up = cgmath::Vector3 {
            x: self.up[0],
            y: self.up[1],
            z: self.up[2]
        };

        self.view_matrix = cgmath::Matrix4::look_at_dir(eye, direction, up);
    }

    pub fn calculate_projection_matrix(&mut self, width: f32, height: f32)
    {
        let fov: f32 = 90.0;
        let aspect: f32 = height / width;
        self.projection_matrix = cgmath::perspective(cgmath::Rad { 0: fov.to_radians() }, aspect, 0.1 as f32, 1024.0 as f32);
    }

    pub fn change_position(&mut self, offset_x: f32, offset_y: f32, offset_z: f32)
    {
        self.position.x += offset_x;
        self.position.y += offset_y;
        self.position.z += offset_z;

        /*self.position[0] += self.yaw.to_radians().sin() * mul_x;
        self.position[1] += offset_y;
        self.position[2] += self.yaw.to_radians().cos() * mul_z;*/
        self.calculate_view_matrix();
    }

    pub fn change_direction(&mut self, pos_x: f32, pos_y: f32)
    {
        self.yaw += (pos_x - self.last_mouse_position[0]) * self::SENSITIVITY;
        self.pitch += (self.last_mouse_position[1] - pos_y) * self::SENSITIVITY;

        if self.pitch > 89.0 {
            self.pitch = 89.0;
        }

        if self.pitch < -89.0 {
            self.pitch = -89.0;
        }

        self.last_mouse_position = [pos_x, pos_y];

        self.direction[0] = -self.yaw.to_radians().cos() * self.pitch.to_radians().cos();
        self.direction[1] = self.pitch.to_radians().sin();
        self.direction[2] = self.yaw.to_radians().sin() * self.pitch.to_radians().cos();

        self.calculate_view_matrix();
    }

}

impl std::fmt::Display for Camera {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(position: {:?})", self.position)
    }
}

impl std::fmt::Debug for Camera {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(position: {:?})", self.position)
    }
}
