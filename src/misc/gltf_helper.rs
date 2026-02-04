use crate::basics::core::Vertex;

pub fn get_model_data() -> Option<(Vec<Vertex>, Vec<u16>)> {
    let (gltf, buffers, _images) =
        gltf::import("assets/monkey.gltf").expect("Failed to import monkey.gltf");

    if let Some(scene) = gltf.scenes().next() {
        if let Some(node) = scene.nodes().next() {
            if let Some(mesh) = node.mesh() {
                println!(
                    "Node #{} - Mesh: {}",
                    node.index(),
                    mesh.name().unwrap_or("Unnamed")
                );

                if let Some(primitive) = mesh.primitives().next() {
                    let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));

                    if let Some(positions) = reader.read_positions() {
                        println!("positions: {}", positions.count());
                    }
                    if let Some(normals) = reader.read_normals() {
                        println!("normals: {}", normals.count());
                    }

                    let mut vertices = vec![];
                    if let Some(positions) = reader.read_positions() {
                        if let Some(normals) = reader.read_normals() {
                            let pairs = positions.zip(normals);
                            for p in pairs {
                                vertices.push(Vertex {
                                    position: [p.0[0], p.0[1], -p.0[2]],
                                    color: [1.0; 3],
                                    normal: [p.1[0], p.1[1], -p.1[2]],
                                    uv: [1.0; 2],
                                });
                            }
                        }
                    }

                    let mut indices = vec![];
                    if let Some(order) = reader.read_indices() {
                        for index in order.into_u32() {
                            indices.push(index as u16);
                        }
                    }

                    return Some((vertices, indices));
                }
            }
        }
    }

    return None;
}
