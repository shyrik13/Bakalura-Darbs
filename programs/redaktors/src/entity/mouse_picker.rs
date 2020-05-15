use crate::entity::camera::Camera;
use crate::entity::object::Object;

use crate::cgmath::*;

/*const RAY_RANGE: u32 = 600;
const RECURSION_COUNT: u32 = 200;

pub fn binanry_search()
{

}

pub fn is_intersection_in_range(camera: &Camera, start: f32, finish: f32, ray: Vector3<f32>) -> bool
{
    let start_point = point_on_ray(camera, ray, start);
    let end_point = point_on_ray(camera, ray, finish);

}

pub fn is_under(object: &Object) -> bool
{

}*/

/*pub fn point_on_ray(camera: &Camera, ray: Vector3<f32>, distance: f32) -> Vector3<f32>
{
    let start = Vector3 {
        x: camera.position[0],
        y: camera.position[1],
        z: camera.position[2]
    };

    let scaled_ray = Vector3 {
        x: ray.x * distance,
        y: ray.y * distance,
        z: ray.z * distance
    };

    return start.sum(scaled_ray)
}*/

pub fn calculate_mouse_ray(camera: &Camera, display_coords: [f32; 2]) -> [f32; 3]
{
    let normalized_coords = normalize_device_coords(camera, display_coords);

    let clip_coords = Vector3{
        x: normalized_coords[0],
        y: normalized_coords[1],
        z: -1.0
    };

    let eye_coords = to_eye_coords(camera, clip_coords);
    let world_ray = to_worlds_coords(camera, eye_coords);

    return [world_ray.x, world_ray.y, world_ray.z]
}

pub fn to_worlds_coords(camera: &Camera, eye_coords: Vector4<f32>) -> Vector3<f32>
{

    let inverted_view = camera.view_matrix.invert().unwrap();

    let ray_world = inverted_view.transform_vector(Vector3 { x: eye_coords.x, y: eye_coords.y, z: eye_coords.z });

    return ray_world.normalize()
}

pub fn normalize_device_coords(camera: &Camera, display_coords: [f32; 2]) -> [f32; 2]
{
    let x: f32 = (2.0 * camera.last_mouse_position[0]) / display_coords[0] - 1.0;
    let y: f32 = 1.0 - (2.0 * camera.last_mouse_position[1]) / display_coords[1];
    return [x, y]
}

pub fn to_eye_coords(camera: &Camera, clip_coords: Vector3<f32>) -> Vector4<f32>
{
    let mut projection_matrix: Matrix4<f32> = Matrix4::new(
        camera.projection_matrix[0][0],
        camera.projection_matrix[0][1],
        camera.projection_matrix[0][2],
        camera.projection_matrix[0][3],
        camera.projection_matrix[1][0],
        camera.projection_matrix[1][1],
        camera.projection_matrix[1][2],
        camera.projection_matrix[1][3],
        camera.projection_matrix[2][0],
        camera.projection_matrix[2][1],
        camera.projection_matrix[2][2],
        camera.projection_matrix[2][3],
        camera.projection_matrix[3][0],
        camera.projection_matrix[3][1],
        camera.projection_matrix[3][2],
        camera.projection_matrix[3][3],
    );

    let inverted_projection = projection_matrix.invert().unwrap();

    let eye_coords = inverted_projection.transform_vector(clip_coords);

    return Vector4 {
        x: eye_coords.x,
        y: eye_coords.y,
        z: -1.0,
        w: 0.0
    }

}

pub fn is_ray_intersect_triangle(vertex_pos_vec: &Vec<Vector3<f32>>, camera_position: Vector3<f32>, mouse_ray: Vector3<f32>, t: f32) -> bool
{

    let ray_point: Vector3<f32> = camera_position + mouse_ray * t;

    println!("vertex_pos_vec {:?}", vertex_pos_vec);
    println!("{:?} {:?} {:?} {:?}", ray_point, camera_position, mouse_ray, t);

    return
        same_side(ray_point, vertex_pos_vec[0], vertex_pos_vec[1], vertex_pos_vec[2]) &&
            same_side(ray_point, vertex_pos_vec[1], vertex_pos_vec[0], vertex_pos_vec[2]) &&
            same_side(ray_point, vertex_pos_vec[2], vertex_pos_vec[0], vertex_pos_vec[1])

}

fn same_side (p1: Vector3<f32>, p2: Vector3<f32>, a: Vector3<f32>, b: Vector3<f32>) -> bool
{
    let cp1: Vector3<f32> = (b-a).cross((p1 - a));
    let cp2: Vector3<f32> = (b-a).cross((p2 - a));
    return cp1.dot(cp2) >= 0.0
}
