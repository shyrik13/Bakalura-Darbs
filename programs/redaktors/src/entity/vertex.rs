implement_vertex!(Vertex, position, tex_coords, normal);
#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
    pub normal: [f32; 3],
}

impl Vertex {
    pub fn new(position: [f32; 3], tex_coords: [f32; 2], normal: [f32; 3]) -> Vertex {
        Vertex { position: position, tex_coords: tex_coords, normal: normal }
    }
}

impl std::fmt::Display for Vertex {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(position: {:?}, tex_coords: {:?}, normal: {:?})", self.position, self.tex_coords, self.normal)
    }
}

impl std::fmt::Debug for Vertex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(position: {:?}, tex_coords: {:?}, normal: {:?})", self.position, self.tex_coords, self.normal)
    }
}