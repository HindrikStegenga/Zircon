use std::{collections::HashMap, io::Write, primitive};

use clap::*;
use gltf::buffer::{self, Source, Data};
use mesh::*;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    input_file: String,
    #[clap(short, long, default_value = ".")]
    output_directory: String,
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    println!(
        "Work dir:   {}",
        std::env::current_dir()
            .expect("Unable to get working directory.")
            .as_os_str()
            .to_string_lossy()
    );
    println!("Input file: {}", args.input_file);
    println!("Output dir: {}", args.output_directory);
    let (document, buffers, _) = gltf::import(args.input_file)?;

    let mesh_count = document.meshes().count();
    for (i, mesh) in document.meshes().enumerate() {
        println!(
            "Processing mesh: {} [{}/{}]",
            mesh.name().unwrap_or("[nameless]"),
            i + 1,
            mesh_count
        );
        let converted = convert_mesh(&document, &mesh, &buffers)?;
        let value = serde_yaml::to_string(&converted)?;
        let filename = &std::format!(
            "{}/{}-{}.yml",
            args.output_directory,
            mesh.name().unwrap_or("out"),
            i + 1
        );
        println!("Writing file: {}", filename);
        let mut file = std::fs::File::create(std::path::Path::new(filename))?;
        file.write(value.as_bytes())?;
    }
    Ok(())
}

fn map_accessor_to_format(accessor: &gltf::Accessor) -> BufferElementFormat {
    match accessor.dimensions() {
        gltf::accessor::Dimensions::Scalar => match accessor.data_type() {
            gltf::accessor::DataType::I8 => BufferElementFormat::I8x1,
            gltf::accessor::DataType::U8 => BufferElementFormat::U8x1,
            gltf::accessor::DataType::I16 => BufferElementFormat::I16x1,
            gltf::accessor::DataType::U16 => BufferElementFormat::U16x1,
            gltf::accessor::DataType::U32 => BufferElementFormat::U32x1,
            gltf::accessor::DataType::F32 => BufferElementFormat::I32x1,
        },
        gltf::accessor::Dimensions::Vec2 => match accessor.data_type() {
            gltf::accessor::DataType::I8 => BufferElementFormat::I8x2,
            gltf::accessor::DataType::U8 => BufferElementFormat::U8x2,
            gltf::accessor::DataType::I16 => BufferElementFormat::I16x2,
            gltf::accessor::DataType::U16 => BufferElementFormat::U16x2,
            gltf::accessor::DataType::U32 => BufferElementFormat::U32x2,
            gltf::accessor::DataType::F32 => BufferElementFormat::I32x2,
        },
        gltf::accessor::Dimensions::Vec3 => match accessor.data_type() {
            gltf::accessor::DataType::I8 => BufferElementFormat::I8x3,
            gltf::accessor::DataType::U8 => BufferElementFormat::U8x3,
            gltf::accessor::DataType::I16 => BufferElementFormat::I16x3,
            gltf::accessor::DataType::U16 => BufferElementFormat::U16x3,
            gltf::accessor::DataType::U32 => BufferElementFormat::U32x3,
            gltf::accessor::DataType::F32 => BufferElementFormat::I32x3,
        },
        gltf::accessor::Dimensions::Vec4 => match accessor.data_type() {
            gltf::accessor::DataType::I8 => BufferElementFormat::I8x4,
            gltf::accessor::DataType::U8 => BufferElementFormat::U8x4,
            gltf::accessor::DataType::I16 => BufferElementFormat::I16x4,
            gltf::accessor::DataType::U16 => BufferElementFormat::U16x4,
            gltf::accessor::DataType::U32 => BufferElementFormat::U32x4,
            gltf::accessor::DataType::F32 => BufferElementFormat::I32x4,
        },
        gltf::accessor::Dimensions::Mat2 => BufferElementFormat::F32x4,
        gltf::accessor::Dimensions::Mat3 => BufferElementFormat::F32x9,
        gltf::accessor::Dimensions::Mat4 => BufferElementFormat::F32x16,
    }
}

fn map_mode(mode: gltf::mesh::Mode) -> mesh::PrimitiveRenderingMode {
    match mode {
        gltf::mesh::Mode::Points => PrimitiveRenderingMode::Points,
        gltf::mesh::Mode::Lines => PrimitiveRenderingMode::Lines,
        gltf::mesh::Mode::LineLoop => PrimitiveRenderingMode::LineLoop,
        gltf::mesh::Mode::LineStrip => PrimitiveRenderingMode::LineStrip,
        gltf::mesh::Mode::Triangles => PrimitiveRenderingMode::Triangles,
        gltf::mesh::Mode::TriangleStrip => PrimitiveRenderingMode::TriangleStrip,
        gltf::mesh::Mode::TriangleFan => PrimitiveRenderingMode::TriangleFan,
    }
}

fn convert_mesh(
    document: &gltf::Document,
    mesh: &gltf::Mesh,
    binary_buffers: &[Data],
) -> std::result::Result<mesh::Mesh, Box<dyn std::error::Error>> {
    let mut primitives = Vec::<Primitive>::new();
    let mut views = Vec::<BufferView>::new();
    let mut buffers = Vec::<Buffer>::new();
    let mut accessors = Vec::<Accessor>::new();

    // Map gltf bufs to actual bufs.
    let mut used_accessors = HashMap::<usize, u32, _>::new();
    let mut used_buffers = HashMap::<usize, u32, _>::new();
    let mut used_views = HashMap::<usize, u32, _>::new();

    let mut add_accessor =
        |accessor: &gltf::Accessor| -> std::result::Result<u32, Box<dyn std::error::Error>> {
            if let Some(value) = used_accessors.get(&accessor.index()) {
                return Ok(*value);
            }
            // We do not re-use an accessor, create a new one!
            let Some(view) = accessor.view() else { return Err("GLTF: sparse accessors are not supported!".into()) };
            if let Some(view_index) = used_views.get(&view.index()) {
                // We did not have an accessor, but have it's view.
                let new_ac = Accessor::new(
                    *view_index,
                    accessor.offset() as u32,
                    accessor.count() as u32,
                    map_accessor_to_format(accessor),
                );
                accessors.push(new_ac);
                used_accessors.insert(accessor.index(), accessors.len() as u32);
                return Ok(accessors.len() as u32);
            }
            // We do not have an accessor or it's view, create a new one!
            if let Some(buffer_index) = used_buffers.get(&view.buffer().index()) {
                // We do have a buffer, so just create a accessor & view.
                let stride = view.stride().map(|e|e as u32);
                let new_vi = BufferView::new(
                    *buffer_index,
                    view.offset() as u32,
                    view.length() as u32,
                    stride,
                );
                views.push(new_vi);
                used_views.insert(view.index(), views.len() as u32);
                let new_ac = Accessor::new(
                    views.len() as u32,
                    accessor.offset() as u32,
                    accessor.count() as u32,
                    map_accessor_to_format(accessor),
                );
                accessors.push(new_ac);
                used_accessors.insert(accessor.index(), accessors.len() as u32);
            }
            // We do not have an accessor, it's view or that ones buffer, create a new one!
            if let Source::Uri(_) = view.buffer().source() {
                return Err("GLTF: Uri buffers are not supported!".into());
            }
            let new_buf = Buffer::new(Vec::from(binary_buffers[view.buffer().index()].0.as_slice()));
            buffers.push(new_buf);
            used_buffers.insert(view.buffer().index(), buffers.len() as u32);

            let stride = view.stride().map(|e|e as u32);
            let new_vi = BufferView::new(
                buffers.len() as u32,
                view.offset() as u32,
                view.length() as u32,
                stride,
            );
            views.push(new_vi);
            used_views.insert(view.index(), views.len() as u32);
            let new_ac = Accessor::new(
                views.len() as u32,
                accessor.offset() as u32,
                accessor.count() as u32,
                map_accessor_to_format(accessor),
            );
            accessors.push(new_ac);
            used_accessors.insert(accessor.index(), accessors.len() as u32);
            Ok(accessors.len() as u32)
        };

    for primitive in mesh.primitives() {
        let render_mode = map_mode(primitive.mode());
        let indices = match primitive.indices() {
            Some(index_accessor) => Some(add_accessor(&index_accessor)?),
            None => None,
        };
        primitives.push(Primitive::new(vec![], indices, render_mode));
    }

    Ok(mesh::Mesh::new(buffers, views, accessors, primitives))
}
