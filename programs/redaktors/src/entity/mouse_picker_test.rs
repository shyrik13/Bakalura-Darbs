use crate::entity::camera::Camera;
use crate::entity::object::Object;
use crate::entity::vertex::Vertex;

use crate::cgmath::*;
use crate::cgmath::num_traits::*;

#[derive(Clone)]
pub struct MouseRay {
    pub position: Vector3<f32>,
    pub direction: Vector3<f32>,
    pub object: Object
}

impl MouseRay {

    pub fn new(camera: &Camera, display_coords: [f32; 2]) -> MouseRay
    {
        let mouse_x: f32 = (2.0 * camera.last_mouse_position[0]) / display_coords[0] - 1.0;
        let mouse_y: f32 = (2.0 * camera.last_mouse_position[1]) / display_coords[1] - 1.0;

        let proj = camera.projection_matrix;
        let view = camera.view_matrix;

        let mut inv_vp: Matrix4<f32> = proj * view;
        inv_vp.invert();

        let screen_pos = Vector4 {
            x: mouse_x,
            y: mouse_y * -1.0,
            z: 1.0,
            w: 1.0
        };

        let world_pos: Vector4<f32> = inv_vp * screen_pos;
        let world_pos_3d = Vector3 {
            x: world_pos.x,
            y: world_pos.y,
            z: world_pos.z
        };

        let dir = world_pos_3d.normalize();

        let model =
            [
                [0.0,0.0,0.0,0.0],
                [0.0,0.0,0.0,0.0],
                [0.0,0.0,0.0,0.0],
                [0.0,0.0,0.0,0.0]
            ];

        return MouseRay {
            position: world_pos_3d,
            direction: dir,
            object: Object {
                model: model,
                id: -1,
                x: 0.0,
                y: 0.0,
                z: 0.0,
                name: ""
            }
        };

    }

    pub fn is_ray_intersect_triangle(&self, vertex_pos1: [f32; 3], vertex_pos2: [f32; 3], vertex_pos3: [f32; 3], object_pos: [f32; 3]) -> f32
    {

        let mouse_direction = self.direction;
        let mouse_position: Vector3<f32> = self.position;

       //let ray_start_position = self.position;
       //let mouse_position = ray_start_position + self.direction * 10.0;

        let vec_vertex_pos1: Vector3<f32> = Vector3 {
            x: vertex_pos1[0]/* + object_pos[0]*/,
            y: vertex_pos1[1]/* + object_pos[1]*/,
            z: vertex_pos1[2]/* + object_pos[2]*/
        };

        let vec_vertex_pos2: Vector3<f32> = Vector3 {
            x: vertex_pos2[0],
            y: vertex_pos2[1],
            z: vertex_pos2[2]
        };

        let vec_vertex_pos3: Vector3<f32> = Vector3 {
            x: vertex_pos3[0],
            y: vertex_pos3[1],
            z: vertex_pos3[2]
        };

        //println!("{:?} {:?} {:?}", vec_vertex_pos1, vec_vertex_pos2, vec_vertex_pos3);

        let edge1 = vec_vertex_pos2 - vec_vertex_pos1;
        let edge2 = vec_vertex_pos3 - vec_vertex_pos1;

        let direction_cross_edge2: Vector3<f32> = mouse_direction.cross(edge2);

        //println!("direction_cross_edge2 {:?}", direction_cross_edge2);

        let determ: f32 = direction_cross_edge2.dot(edge1);

        //println!("determ {:?}", determ);

        if determ > -0.0000001 && determ < 0.0000001 {
            return std::f32::MAX;
        }

        let inverse_determ = 1.0 / determ;

        //println!("inverse_determ {:?}", inverse_determ);

        let distance_vector: Vector3<f32> = mouse_position - vec_vertex_pos1;

        //println!("distance_vector {:?}", distance_vector);

        let mut triangle_u: f32 = direction_cross_edge2.dot(distance_vector);

        //println!("triangle_u {:?}", triangle_u);

        triangle_u = triangle_u * inverse_determ;

        //println!("triangle_u {:?}", triangle_u);

        if triangle_u < 0.0 || triangle_u > 1.0 {
            return std::f32::MAX;
        }

        let distance_cross_edge1: Vector3<f32> = distance_vector.cross(edge1);

        //println!("distance_cross_edge1 {:?}", distance_cross_edge1);

        let mut triangle_v: f32 = mouse_direction.dot(distance_cross_edge1);

        //println!("triangle_v {:?}", triangle_v);

        triangle_v = triangle_v * inverse_determ;

        //println!("triangle_v {:?}", triangle_v);

        //println!("triangle_u + triangle_v {:?}", triangle_u + triangle_v);
        if triangle_v < 0.0 || triangle_u + triangle_v > 1.0 {
            return std::f32::MAX;
        }

        let mut ray_distance: f32 = distance_cross_edge1.dot(edge2);

        //println!("ray_distance {:?}", ray_distance);

        ray_distance *= inverse_determ;

        //println!("ray_distance {:?}", ray_distance);

        if ray_distance < 0.0 {
            return std::f32::MAX;
        }

        return ray_distance
    }

}

impl std::fmt::Display for MouseRay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(position: {:?} direction: {:?})", self.position, self.direction)
    }
}

impl std::fmt::Debug for MouseRay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(position: {:?} direction: {:?})", self.position, self.direction)
    }
}
