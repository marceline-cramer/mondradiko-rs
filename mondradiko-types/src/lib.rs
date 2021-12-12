pub struct AssetId(u32);
pub struct AssetType(u32);
pub struct ComponentId(u32);
pub struct EntityId(u32);

pub mod components {
    use super::*;

    pub struct PositionComponent {
        pub position: [f32; 3],
    }

    pub struct MeshComponent {
        pub mesh: AssetId,
    }
}

pub mod assets {
    pub mod mesh {
        pub struct Vertex {
            position: [f32; 3],
        }

        pub type Index = u32;

        pub struct MeshAsset {
            pub vertices: Vec<Vertex>,
            pub index: Vec<Vertex>,
        }
    }
}
