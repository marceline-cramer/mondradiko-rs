pub use bytemuck;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(transparent)]
pub struct AssetId(pub u32);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(transparent)]
pub struct AssetType(pub u32);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(transparent)]
pub struct ComponentId(pub u32);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(transparent)]
pub struct EntityId(pub u32);

pub mod events {
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
    #[repr(C)]
    pub struct UpdateEvent {
        // TODO delta-time member
    }
}

pub mod components {
    use super::*;

    #[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
    #[repr(C)]
    pub struct PositionComponent {
        pub position: [f32; 3],
    }

    #[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
    #[repr(C)]
    pub struct MeshComponent {
        pub mesh: AssetId,
    }
}

pub mod assets {
    pub mod mesh {
        pub struct Vertex {
            pub position: [f32; 3],
        }

        pub type Index = u32;

        pub struct MeshAsset {
            pub vertices: Vec<Vertex>,
            pub indices: Vec<Index>,
        }
    }
}
