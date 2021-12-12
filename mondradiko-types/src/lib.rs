pub use bytemuck;

use bytemuck::{Pod, Zeroable};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Pod, Zeroable)]
#[repr(transparent)]
pub struct AssetId(pub u32);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Pod, Zeroable)]
#[repr(transparent)]
pub struct AssetType(pub u32);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Pod, Zeroable)]
#[repr(transparent)]
pub struct ComponentId(pub u32);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Pod, Zeroable)]
#[repr(transparent)]
pub struct EntityId(pub u32);

pub mod events {
    use super::*;

    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Pod, Zeroable)]
    #[repr(C)]
    pub struct UpdateEvent {
        // TODO delta-time member
    }
}

pub mod components {
    use super::*;

    #[derive(Clone, Copy, Debug, Pod, Zeroable)]
    #[repr(C)]
    pub struct Position {
        pub position: [f32; 3],
    }

    #[derive(Clone, Copy, Debug, Pod, Zeroable)]
    #[repr(C)]
    pub struct Label {
        pub label: AssetId,
    }

    #[derive(Clone, Copy, Debug, Pod, Zeroable)]
    #[repr(C)]
    pub struct Mesh {
        pub mesh: AssetId,
    }
}

pub mod assets {
    use super::*;

    pub mod mesh {
        use super::*;

        #[derive(Clone, Debug, Deserialize, Serialize)]
        pub struct Vertex {
            pub position: [f32; 3],
        }

        pub type Index = u32;

        #[derive(Clone, Debug, Deserialize, Serialize)]
        pub struct MeshAsset {
            pub vertices: Vec<Vertex>,
            pub indices: Vec<Index>,
        }
    }

    pub mod label {
        use super::*;

        #[derive(Clone, Debug, Deserialize, Serialize)]
        pub struct LabelAsset {
            pub title: String,
        }
    }
}
