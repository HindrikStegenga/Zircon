use clap::Parser;
use mesh::*;
use obj::{load_obj, Obj};
use std::fs;
use std::fs::File;
use std::io::BufReader;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input_file: String,
    #[arg(short, long)]
    output_file: String,
}

fn main() {
    let args = Args::parse();
    let input = BufReader::new(File::open(args.input_file).expect("Could not open file"));
    let model: Obj = load_obj(input).expect("Could parse the file as OBJ.");

    let mut primitive = Primitive {
        buffers: vec![],
        bindings: vec![],
        rendering_mode: RenderingMode::TriangleStrip,
        index_buffer_binding: None,
    };

    if !model.indices.is_empty() {
        let buffer_index = primitive.buffers.len() as u32;
        let mut buffer = vec![];
        for index in model.indices {
            let bytes: [u8; 2] = index.to_ne_bytes();
            buffer.push(bytes[0]);
            buffer.push(bytes[1]);
        }
        primitive.buffers.push(buffer);

        primitive.index_buffer_binding = Some(BufferBinding {
            attributes: vec![BufferAttribute {
                location: 0,
                format: BufferElementFormat::I16x1,
                offset_in_bytes: 0,
            }],
            buffer_index,
            stride_in_bytes: std::mem::size_of::<u16>() as u32,
            input_rate: InputRate::PerVertex,
        });
    }

    if !model.vertices.is_empty() {
        let buffer_index = primitive.buffers.len() as u32;
        primitive.bindings.push(BufferBinding {
            attributes: vec![
                BufferAttribute {
                    location: 0,
                    format: BufferElementFormat::F32x3,
                    offset_in_bytes: 0,
                },
                BufferAttribute {
                    location: 1,
                    format: BufferElementFormat::F32x3,
                    offset_in_bytes: std::mem::size_of::<[f32; 3]>() as u32,
                },
            ],
            buffer_index,
            stride_in_bytes: (std::mem::size_of::<[f32; 3]>() * 2) as u32,
            input_rate: InputRate::PerVertex,
        });
        let mut buffer = vec![];
        for vertex in model.vertices {
            let position = vertex.position;
            let normal = vertex.normal;
            let mut push_bytes = |val: [f32; 3]| -> () {
                for float in val {
                    let bytes = float.to_ne_bytes();
                    buffer.push(bytes[0]);
                    buffer.push(bytes[1]);
                    buffer.push(bytes[2]);
                    buffer.push(bytes[3]);
                }
            };
            push_bytes(position);
            push_bytes(normal);
        }
        primitive.buffers.push(buffer);
    }

    let toml_output = toml::to_string_pretty(&primitive).expect("Could not convert into TOML.");
    fs::write(args.output_file, toml_output.as_bytes()).expect("Could not write output.");
}
