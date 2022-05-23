use clap::*;
use gltf::*;
use graphyte_mesh::{AttributePurpose, BufferElementFormat, BufferView};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    input_file: String,
    #[clap(short, long, default_value = "out")]
    output_file: String,
}

fn main() {
    let args = Args::parse();

    println!("{}", args.input_file);
    println!("{}", args.output_file);

    match try_extract_mesh(args.input_file, args.output_file) {
        Ok(()) => (),
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

fn map_accessor_to_format(accessor: &Accessor) -> BufferElementFormat {
    match accessor.dimensions() {
        accessor::Dimensions::Scalar => match accessor.data_type() {
            accessor::DataType::I8 => BufferElementFormat::I8x1,
            accessor::DataType::U8 => BufferElementFormat::U8x1,
            accessor::DataType::I16 => BufferElementFormat::I16x1,
            accessor::DataType::U16 => BufferElementFormat::U16x1,
            accessor::DataType::U32 => BufferElementFormat::U32x1,
            accessor::DataType::F32 => BufferElementFormat::I32x1,
        },
        accessor::Dimensions::Vec2 => match accessor.data_type() {
            accessor::DataType::I8 => BufferElementFormat::I8x2,
            accessor::DataType::U8 => BufferElementFormat::U8x2,
            accessor::DataType::I16 => BufferElementFormat::I16x2,
            accessor::DataType::U16 => BufferElementFormat::U16x2,
            accessor::DataType::U32 => BufferElementFormat::U32x2,
            accessor::DataType::F32 => BufferElementFormat::I32x2,
        },
        accessor::Dimensions::Vec3 => match accessor.data_type() {
            accessor::DataType::I8 => BufferElementFormat::I8x3,
            accessor::DataType::U8 => BufferElementFormat::U8x3,
            accessor::DataType::I16 => BufferElementFormat::I16x3,
            accessor::DataType::U16 => BufferElementFormat::U16x3,
            accessor::DataType::U32 => BufferElementFormat::U32x3,
            accessor::DataType::F32 => BufferElementFormat::I32x3,
        },
        accessor::Dimensions::Vec4 => match accessor.data_type() {
            accessor::DataType::I8 => BufferElementFormat::I8x4,
            accessor::DataType::U8 => BufferElementFormat::U8x4,
            accessor::DataType::I16 => BufferElementFormat::I16x4,
            accessor::DataType::U16 => BufferElementFormat::U16x4,
            accessor::DataType::U32 => BufferElementFormat::U32x4,
            accessor::DataType::F32 => BufferElementFormat::I32x4,
        },
        accessor::Dimensions::Mat2 => BufferElementFormat::F32x4,
        accessor::Dimensions::Mat3 => BufferElementFormat::F32x9,
        accessor::Dimensions::Mat4 => BufferElementFormat::F32x16,
    }
}

fn map_semantic(semantic: &Semantic) -> AttributePurpose {
    match semantic {
        Semantic::Positions => AttributePurpose::Position,
        Semantic::Normals => AttributePurpose::Normals,
        Semantic::Tangents => AttributePurpose::Tangents,
        Semantic::Colors(_) => AttributePurpose::Colors,
        Semantic::TexCoords(_) => AttributePurpose::TexCoords,
        Semantic::Joints(_) => AttributePurpose::Undefined,
        Semantic::Weights(_) => AttributePurpose::Undefined,
    }
}

fn try_extract_mesh(
    input: String,
    output: String,
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let (document, buffers, images) = gltf::import(input)?;

    let mut counter = 0;
    for mesh in document.meshes() {
        for primitive in mesh.primitives() {
            rebuild_buffers_views_and_accessors(&buffers, &document, primitive);
        }

        // let out_mesh = todo!();
        // //remap_primitive_data(&primitive)?;
        // let binary = serde_yaml::to_vec(&out_mesh)?;

        // let out_name: String = match mesh.name() {
        //     Some(v) => format!("{}_{}_{}.bin", output, counter, v),
        //     None => format!("{}_{}.bin", output, counter),
        // };
        // let mut output_file = File::create(out_name)?;
        // output_file.write_all(&binary)?;
        // output_file.flush()?;

        counter += 1;
    }

    Ok(())
}

fn rebuild_buffers_views_and_accessors(
    buffers: &Vec<gltf::buffer::Data>,
    document: &gltf::Document,
    primitive: gltf::Primitive,
) {
    let mut accessors = primitive.attributes().map(|(_, a)| a).collect::<Vec<_>>();
    if let Some(indices) = primitive.indices() {
        accessors.push(indices);
    }
    let buffer_views: Vec<gltf::buffer::View> = accessors
        .iter()
        .filter_map(|a| a.view())
        .collect::<Vec<_>>();

    let mut new_buffer: Vec<u8> = vec![];
    let mut new_views: Vec<(BufferView, gltf::buffer::View)> = vec![];
    let mut new_accessors: Vec<(graphyte_mesh::Accessor, gltf::Accessor)> = vec![];

    for view in buffer_views {
        let buffer = buffers[view.buffer().index()].0.as_slice();
        let offset = new_buffer.len();
        new_buffer.extend_from_slice(&buffer[view.offset()..view.offset() + view.length()]);
        new_views.push((
            BufferView::new(
                0,
                offset as u32,
                view.length() as u32,
                view.stride().map(|f| f as u32),
            ),
            view,
        ));
    }
    for accessor in accessors {
        let new_view: (usize, &(BufferView, gltf::buffer::View)) = new_views
            .iter()
            .enumerate()
            .find(|&(_, (_, b))| b.index() == accessor.view().unwrap().index())
            .unwrap();
        new_accessors.push((
            graphyte_mesh::Accessor::new(
                new_view.0 as u32,
                accessor.offset() as u32,
                accessor.count() as u32,
                map_accessor_to_format(&accessor),
            ),
            accessor,
        ));
    }
}
