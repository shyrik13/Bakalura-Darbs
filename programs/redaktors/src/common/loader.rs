use std::io::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::entity::Vertex;
use std::io::Cursor;

const SHADER_PATH: &'static str = "src/res/shader/";
const FILES_PATH: &'static str = "src/res/files/";
const TEXTURE_PATH: &'static str = "src/res/texture/";

pub fn load_into_vertex_vector(filename: &str) -> Vec<Vertex> {
    let file: File = File::open(FILES_PATH.to_owned() + filename).unwrap();
    let reader = BufReader::new(file);

    let mut vec_positions: Vec<[f32; 3]> = Vec::new();
    let mut vec_textures: Vec<[f32; 2]> = Vec::new();
    let mut vec_normals: Vec<[f32; 3]> = Vec::new();
    let mut vec_vertex: Vec<Vertex> = Vec::new();

    // Dummy
    vec_positions.push( [0.0, 0.0, 0.0]);
    vec_textures.push([0.0, 0.0]);
    vec_normals.push([0.0, 0.0, 0.0]);

    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let vec_split: Vec<&str> = line.split_whitespace().collect();
        if vec_split[0] == "v" {
            let v1: f32 = vec_split[1].parse().unwrap();
            let v2: f32 = vec_split[2].parse().unwrap();
            let v3: f32 = vec_split[3].parse().unwrap();
            vec_positions.push([v1, v2, v3]);
        } else if vec_split[0] == "vt" {
            let v1: f32 = vec_split[1].parse().unwrap();
            let v2: f32 = vec_split[2].parse().unwrap();
            vec_textures.push([v1, v2]);
        } else if vec_split[0] == "vn" {
            let v1: f32 = vec_split[1].parse().unwrap();
            let v2: f32 = vec_split[2].parse().unwrap();
            let v3: f32 = vec_split[3].parse().unwrap();
            vec_normals.push([v1, v2, v3]);
        } else if vec_split[0] == "f" {
            for i in 1..vec_split.len() {
                let vec_split: Vec<&str> = vec_split[i].split("/").collect();
                let ind1: usize = vec_split[0].parse().unwrap();
                let ind2: usize = vec_split[1].parse().unwrap();
                let ind3: usize = vec_split[2].parse().unwrap();
                let vertex: Vertex = Vertex::new(vec_positions[ind1], vec_textures[ind2], vec_normals[ind3]);
                vec_vertex.push(vertex);
            }
        }
    }
    vec_vertex
}

pub fn load_string(filename: &str) -> String
{
    let mut file = File::open(SHADER_PATH.to_owned() + filename).expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");
    contents
}

pub fn load_diffuse_texture(object_name: &str) -> glium::texture::RawImage2d<u8>
{
    let path = TEXTURE_PATH.to_owned() + object_name + "-diffuse.jpg";

    let image = image::load(Cursor::new(&load_file_as_byte_vec(&path)[..]),
                            image::ImageFormat::JPEG).unwrap().to_rgba();
    let image_dimensions = image.dimensions();
    glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions)
}

pub fn load_normal_texture(object_name: &str) -> glium::texture::RawImage2d<u8>
{
    let path = TEXTURE_PATH.to_owned() + object_name + "-normal.png";

    let image = image::load(Cursor::new(&load_file_as_byte_vec(&path)[..]),
                            image::ImageFormat::PNG).unwrap().to_rgba();
    let image_dimensions = image.dimensions();
    glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions)
}

fn load_file_as_byte_vec(filename: &str) -> Vec<u8> {
    let mut f = File::open(&filename).expect("no file found");
    let metadata = std::fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    buffer
}