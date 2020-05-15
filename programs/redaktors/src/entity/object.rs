#[derive(Clone)]
pub struct Object {
    pub model: [[f32; 4]; 4],
    pub id: i32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub name: &'static str
}

impl Object {

    pub fn new(name: &'static str) -> Object {
        let model =
            [
                [0.0,0.0,0.0,0.0],
                [0.0,0.0,0.0,0.0],
                [0.0,0.0,0.0,0.0],
                [0.0,0.0,0.0,0.0]
            ];
        Object { model: model, id: -1, x: 0.0, y: 0.0, z: 0.0, name: name }
    }

    pub fn set_x_y_z(&mut self, x: f32, y: f32, z: f32) {
        self.x = x;
        self.y = y;
        self.z = z;
    }

    pub fn init_gl_object_model(&mut self, c: f32, s: f32) {
        self.model = [
            [c.powi(2), -c*s, s, 0.0],
            [c*(s.powi(2)+s), c.powi(2)-s.powi(3), -c*s, 0.0],
            [s*(s-c.powi(2)), c*(s.powi(2)+s), c.powi(2), 0.0],
            [self.x, self.y, self.z, 1.0f32]
        ]
    }
}

impl std::fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(name: {:?}, model: {:?})", self.name, self.model)
    }
}

impl std::fmt::Debug for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(name: {:?}, model: {:?})", self.name, self.model)
    }
}
