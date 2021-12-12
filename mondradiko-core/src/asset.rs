use super::*;
use slab::Slab;
use std::collections::HashMap;

pub trait AssetLoader {
    fn load_asset(&mut self, data: &[u8]) -> Result<AssetId, ()>;
}

#[derive(Default)]
pub struct AssetStore {
    types_by_name: HashMap<String, AssetType>,
    type_index: HashMap<AssetId, AssetType>,
    loaders: Slab<Box<dyn AssetLoader>>,
}

impl AssetStore {
    pub fn add_loader(
        &mut self,
        name: &str,
        loader: Box<dyn AssetLoader>,
    ) -> Result<AssetType, ()> {
        if self.types_by_name.contains_key(name) {
            println!("error: asset loader for type {} already exists", name);
            return Err(());
        }

        let loader_index = self.loaders.insert(loader);
        let asset_type = AssetType(loader_index as u32);
        self.types_by_name.insert(name.to_string(), asset_type);
        Ok(asset_type)
    }

    pub fn get_asset_type(&self, name: &str) -> Result<AssetType, ()> {
        if let Some(asset_type) = self.types_by_name.get(name) {
            Ok(*asset_type)
        } else {
            Err(())
        }
    }

    pub fn load_asset(&mut self, asset_type: AssetType, data: &[u8]) -> Result<AssetId, ()> {
        let loader_index = asset_type.0 as usize;
        if let Some(loader) = self.loaders.get_mut(loader_index) {
            let id = loader.load_asset(data);
            if let Ok(id) = id {
                self.type_index.insert(id, asset_type);
            }
            id
        } else {
            println!("error: no loader for {:?}", asset_type);
            Err(())
        }
    }
}
