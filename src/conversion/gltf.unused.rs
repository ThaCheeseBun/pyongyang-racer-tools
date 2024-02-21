// Support for glTF was considered but scrapped due to format limitations.
// Might be brought back when i bother.

#[derive(Deserialize, Serialize)]
struct glTF {
    #[serde(skip_serializing_if = "Option::is_none")]
    accessors: Option<Vec<glTF_Accessor>>,
    asset: glTF_Asset,
    #[serde(skip_serializing_if = "Option::is_none")]
    buffers: Option<Vec<glTF_Buffer>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bufferViews: Option<Vec<glTF_BufferView>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    materials: Option<Vec<glTF_Material>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    meshes: Option<Vec<glTF_Mesh>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nodes: Option<Vec<glTF_Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scene: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scenes: Option<Vec<glTF_Scene>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    textures: Option<Vec<glTF_Texture>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    images: Option<Vec<glTF_Image>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    samplers: Option<Vec<glTF_Sampler>>,
}

#[derive(Deserialize, Serialize)]
struct glTF_Accessor {
    #[serde(skip_serializing_if = "Option::is_none")]
    bufferView: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    byteOffset: Option<i32>,
    componentType: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    normalized: Option<bool>,
    count: i32,
    #[serde(rename = "type")]
    type_: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<Vec<f32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min: Option<Vec<f32>>,
    // sparse
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
}
#[derive(Deserialize, Serialize)]
struct glTF_Asset {
    #[serde(skip_serializing_if = "Option::is_none")]
    copyright: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    generator: Option<String>,
    version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    minVersion: Option<String>,
}
#[derive(Deserialize, Serialize)]
struct glTF_Buffer {
    #[serde(skip_serializing_if = "Option::is_none")]
    uri: Option<String>,
    byteLength: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
}
#[derive(Deserialize, Serialize)]
struct glTF_BufferView {
    buffer: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    byteOffset: Option<i32>,
    byteLength: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    byteStride: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
}
#[derive(Deserialize, Serialize)]
struct glTF_Material {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pbrMetallicRoughness: Option<glTF_PbrMetallicRoughness>,
    // normalTexture
    // occlusionTexture
    // emissiveTexture
    #[serde(skip_serializing_if = "Option::is_none")]
    emissiveFactor: Option<Vec<f32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    alphaMode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    alphaCutoff: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    doubleSided: Option<bool>,
}
#[derive(Deserialize, Serialize)]
struct glTF_PbrMetallicRoughness {
    #[serde(skip_serializing_if = "Option::is_none")]
    baseColorFactor: Option<[f32; 4]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    baseColorTexture: Option<glTF_TextureInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metallicFactor: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    roughnessFactor: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metallicRoughnessTexture: Option<glTF_TextureInfo>,
}
#[derive(Deserialize, Serialize)]
struct glTF_Mesh {
    primitives: Vec<glTF_Mesh_Primitives>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weights: Option<Vec<f32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
}
#[derive(Deserialize, Serialize)]
struct glTF_Mesh_Primitives {
    attributes: HashMap<String, i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    indices: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    material: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    targets: Option<HashMap<String, String>>,
}
#[derive(Deserialize, Serialize)]
struct glTF_Node {
    #[serde(skip_serializing_if = "Option::is_none")]
    children: Option<Vec<i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    matrix: Option<[f32; 16]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mesh: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rotation: Option<[f32; 4]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scale: Option<[f32; 3]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    translation: Option<[f32; 3]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weights: Option<Vec<f32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
}
#[derive(Deserialize, Serialize)]
struct glTF_Scene {
    #[serde(skip_serializing_if = "Option::is_none")]
    nodes: Option<Vec<i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
}
#[derive(Deserialize, Serialize)]
struct glTF_Texture {
    #[serde(skip_serializing_if = "Option::is_none")]
    sampler: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
}
#[derive(Deserialize, Serialize)]
struct glTF_Image {
    #[serde(skip_serializing_if = "Option::is_none")]
    uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mimeType: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bufferView: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
}
#[derive(Deserialize, Serialize)]
struct glTF_TextureInfo {
    index: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    texCoord: Option<i32>,
}
#[derive(Deserialize, Serialize)]
struct glTF_Sampler {
    #[serde(skip_serializing_if = "Option::is_none")]
    magFilter: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minFilter: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wrapS: Option<i32>, // U
    #[serde(skip_serializing_if = "Option::is_none")]
    wrapT: Option<i32>, // V
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
}

fn __d() {
    let mut geo_buf_w = Cursor::new(vec![]);
    for x in &tris_buf {
        geo_buf_w.write_u16::<LittleEndian>(x.c as u16).unwrap();
        geo_buf_w.write_u16::<LittleEndian>(x.b as u16).unwrap();
        geo_buf_w.write_u16::<LittleEndian>(x.a as u16).unwrap();
    }
    // byte alignment
    let mut cur_pos = geo_buf_w.seek(SeekFrom::Current(0)).unwrap();
    let scalar_len = cur_pos;
    let vtx_offset = cur_pos + cur_pos % 4;
    if cur_pos % 4 != 0 {
        geo_buf_w.write_all(&vec![0u8; cur_pos as usize]).unwrap();
    }

    let fvtx = &frame_buf[0][0];
    let mut vtx_max = vec![fvtx.x, fvtx.y, fvtx.z];
    let mut vtx_min = vec![fvtx.x, fvtx.y, fvtx.z];

    for x in &frame_buf[0] {
        if x.x > vtx_max[0] {
            vtx_max[0] = x.x;
        }
        if x.x < vtx_min[0] {
            vtx_min[0] = x.x;
        }
        geo_buf_w.write_f32::<LittleEndian>(x.x).unwrap();
        if x.y > vtx_max[1] {
            vtx_max[1] = x.y;
        }
        if x.y < vtx_min[1] {
            vtx_min[1] = x.y;
        }
        geo_buf_w.write_f32::<LittleEndian>(x.y).unwrap();
        if x.z > vtx_max[2] {
            vtx_max[2] = x.z;
        }
        if x.z < vtx_min[2] {
            vtx_min[2] = x.z;
        }
        geo_buf_w.write_f32::<LittleEndian>(x.z).unwrap();
    }

    // byte alignment
    cur_pos = geo_buf_w.seek(SeekFrom::Current(0)).unwrap();
    let vtx_len = cur_pos - vtx_offset;
    let uvs_offset = cur_pos + cur_pos % 4;
    if cur_pos % 4 != 0 {
        geo_buf_w.write_all(&vec![0u8; cur_pos as usize]).unwrap();
    }

    for x in &uvs_buf {
        geo_buf_w.write_f32::<LittleEndian>(x.0).unwrap();
        geo_buf_w.write_f32::<LittleEndian>(x.1).unwrap();
    }
    cur_pos = geo_buf_w.seek(SeekFrom::Current(0)).unwrap();
    let uvs_len = cur_pos - uvs_offset;

    let geo_buf = geo_buf_w.into_inner();
    let geo_buf_b64 = Base64::encode_string(&geo_buf);

    let scene = glTF_Scene {
        nodes: Some(vec![0]),
        name: None,
    };
    let node = glTF_Node {
        mesh: Some(0),
        children: None,
        matrix: None,
        rotation: None,
        scale: None,
        translation: None,
        weights: None,
        name: None,
    };
    
    let mut primitives_attr = HashMap::new();
    primitives_attr.insert(String::from("POSITION"), 1);
    primitives_attr.insert(String::from("TEXCOORD_0"), 2);

    let primitives = glTF_Mesh_Primitives {
        attributes: primitives_attr,
        indices: Some(0),
        material: Some(0),
        mode: None,
        targets: None,
    };
    let mesh = glTF_Mesh {
        primitives: vec![primitives],
        weights: None,
        name: None,
    };

    let buffer = glTF_Buffer {
        uri: Some(format!("data:application/octet-stream;base64,{}", geo_buf_b64)),
        byteLength: geo_buf.len() as i32,
        name: None,
    };

    let bufferView0 = glTF_BufferView {
        buffer: 0,
        byteOffset: Some(0),
        byteLength: scalar_len as i32,
        target: Some(34963),
        byteStride: None,
        name: None,
    };
    let bufferView1 = glTF_BufferView {
        buffer: 0,
        byteOffset: Some(vtx_offset as i32),
        byteLength: vtx_len as i32,
        target: Some(34962),
        byteStride: None,
        name: None,
    };
    let bufferView2 = glTF_BufferView {
        buffer: 0,
        byteOffset: Some(uvs_offset as i32),
        byteLength: uvs_len as i32,
        target: Some(34962),
        byteStride: None,
        name: None,
    };

    let accessor0 = glTF_Accessor {
        bufferView: Some(0),
        byteOffset: Some(0),
        componentType: 5123,
        count: (tris_buf.len() * 3) as i32,
        type_: String::from("SCALAR"),
        normalized: None,
        max: None,
        min: None,
        name: None,
    };
    let accessor1 = glTF_Accessor {
        bufferView: Some(1),
        byteOffset: Some(0),
        componentType: 5126,
        count: frame_buf[0].len() as i32,
        type_: String::from("VEC3"),
        normalized: None,
        max: Some(vtx_max),
        min: Some(vtx_min),
        name: None,
    };
    let accessor2 = glTF_Accessor {
        bufferView: Some(2),
        byteOffset: Some(0),
        componentType: 5126,
        count: uvs_buf.len() as i32,
        type_: String::from("VEC2"),
        normalized: None,
        max: None,
        min: None,
        name: None,
    };

    let textureInfo = glTF_TextureInfo {
        index: 0,
        texCoord: None,
    };

    let material_pbr = glTF_PbrMetallicRoughness {
        baseColorTexture: Some(textureInfo),
        baseColorFactor: None,
        metallicFactor: Some(0.0),
        roughnessFactor: Some(1.0),
        metallicRoughnessTexture: None,
    };

    let material = glTF_Material {
        name: None,
        emissiveFactor: None,
        alphaMode: None,
        alphaCutoff: None,
        doubleSided: Some(true),
        pbrMetallicRoughness: Some(material_pbr),
    };

    let texture = glTF_Texture {
        source: Some(0),
        sampler: Some(0),
        name: None,
    };

    let tx_image = glTF_Image {
        uri: Some(String::from("hwuiparam_UV.png")),
        mimeType: None,
        bufferView: None,
        name: None,
    };

    let sampler = glTF_Sampler {
        wrapS: Some(33648),
        wrapT: Some(33648),
        magFilter: Some(9729),
        minFilter: Some(9987),
        name: None,
    };

    let adasd = glTF {
        scene: Some(0),
        scenes: Some(vec![scene]),
        nodes: Some(vec![node]),
        meshes: Some(vec![mesh]),
        buffers: Some(vec![buffer]),
        bufferViews: Some(vec![bufferView0, bufferView1, bufferView2]),
        accessors: Some(vec![accessor0, accessor1, accessor2]),
        asset: glTF_Asset {
            version: String::from("2.0"),
            copyright: None,
            generator: None,
            minVersion: None,
        },
        materials: Some(vec![material]),
        textures: Some(vec![texture]),
        images: Some(vec![tx_image]),
        samplers: Some(vec![sampler]),
    };

    let mut out_fff = fs::File::create("test.gltf").unwrap();
    serde_json::to_writer_pretty(&mut out_fff, &adasd).unwrap();
    out_fff.flush().unwrap();
}
