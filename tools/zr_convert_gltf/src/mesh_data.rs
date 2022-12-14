use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MeshData {
    
}

#[derive(Serialize, Deserialize, Debug)]
struct Primitive {

}

#[derive(Serialize, Deserialize, Debug)]
pub struct Buffer {
    bytes: Vec<u8>
}