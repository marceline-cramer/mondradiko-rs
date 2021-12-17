pub use bincode;

use components::{Label, Position};
use mondradiko_types::*;
use std::cell::{RefCell, RefMut};
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

pub mod asset;
pub mod script;

use asset::*;

#[derive(Clone, Copy)]
pub struct EventType(u32);

pub struct ResourceId(u32);

#[derive(Default)]
struct LabelStore {
    labels: slab::Slab<assets::label::LabelAsset>,
}

struct LabelLoader {
    store: Arc<Mutex<LabelStore>>,
}

impl AssetLoader for LabelLoader {
    fn load_asset(&mut self, data: &[u8]) -> Result<AssetId, ()> {
        if let Ok(asset) = bincode::deserialize(data) {
            let index = self.store.lock().unwrap().labels.insert(asset);
            let id = AssetId(index as u32);
            Ok(id)
        } else {
            Err(())
        }
    }
}

#[derive(Default)]
pub struct Core {
    scripts: RefCell<Vec<Box<dyn script::Instance>>>,
    positions: RefCell<HashMap<EntityId, Position>>,
    labels: RefCell<HashMap<EntityId, Label>>,
    asset_store: RefCell<AssetStore>,
    label_store: Arc<Mutex<LabelStore>>,
}

impl Core {
    pub fn new() -> Self {
        let core = Self::default();

        let label_loader = LabelLoader {
            store: core.label_store.clone(),
        };
        core.asset_store
            .borrow_mut()
            .add_loader("label", Box::new(label_loader))
            .unwrap();

        core
    }

    pub fn add_script(&self, script: Box<dyn script::Instance>) {
        self.scripts.borrow_mut().push(script);
    }

    pub fn step(&self) {
        // TODO bytemuck event casting
        // let event = events::UpdateEvent;
        let event: &[u8] = &[];
        // TODO event type store
        let event_type = EventType(0);

        for script in self.scripts.borrow_mut().iter_mut() {
            script.handle_event(event_type, event);
        }

        let positions = self.positions.borrow();
        let labels = self.labels.borrow();

        let mut entities = std::collections::HashSet::new();

        for e in positions.keys() {
            entities.insert(e);
        }

        for e in labels.keys() {
            entities.insert(e);
        }

        println!("entities:");
        for entity in entities.iter() {
            println!("  {:?}", entity);
            if let Some(position) = positions.get(entity) {
                println!("    {:?}", position);
            }
            if let Some(label) = labels.get(entity) {
                println!("    {:?}", label);
            }
        }
    }
}

pub struct BasicCoreApi {
    core: Arc<Core>,
}

impl BasicCoreApi {
    pub fn new(core: Arc<Core>) -> Self {
        Self { core }
    }

    pub fn generic_write_components<T: bytemuck::Pod>(
        mut dst: RefMut<HashMap<EntityId, T>>,
        entities: &[EntityId],
        data: &[u8],
    ) {
        let src: &[T] = bytemuck::cast_slice(data);
        assert_eq!(src.len(), entities.len());

        for (id, component) in entities.iter().zip(src.iter()) {
            dst.insert(*id, *component);
        }
    }

    pub fn generic_delete_components<T>(
        mut dst: RefMut<HashMap<EntityId, T>>,
        entities: &[EntityId],
    ) {
        for entity in entities.iter() {
            dst.remove(entity);
        }
    }
}

impl script::Api for BasicCoreApi {
    fn get_asset_type(&self, name: &str) -> script::Result<AssetType> {
        self.core.asset_store.borrow().get_asset_type(name)
    }

    fn load_asset(&mut self, asset_type: AssetType, data: &[u8]) -> script::Result<AssetId> {
        self.core
            .asset_store
            .borrow_mut()
            .load_asset(asset_type, data)
    }

    fn get_component_id(&self, name: &str) -> script::Result<ComponentId> {
        match name {
            "position" => Ok(ComponentId(0)),
            "label" => Ok(ComponentId(1)),
            _ => Err(()),
        }
    }

    fn write_components(&mut self, component: ComponentId, entities: &[EntityId], data: &[u8]) {
        match component.0 {
            0 => Self::generic_write_components(self.core.positions.borrow_mut(), entities, data),
            1 => Self::generic_write_components(self.core.labels.borrow_mut(), entities, data),
            _ => println!("no component {:?}", component),
        };
    }

    fn delete_components(&mut self, component: ComponentId, entities: &[EntityId]) {
        match component.0 {
            0 => Self::generic_delete_components(self.core.positions.borrow_mut(), entities),
            1 => Self::generic_delete_components(self.core.labels.borrow_mut(), entities),
            _ => println!("no component {:?}", component),
        };
    }

    fn get_resource_id(&self, name: &str) -> script::Result<ResourceId> {
        unimplemented!()
    }

    fn write_resource(&mut self, id: ResourceId, data: &[u8]) {
        unimplemented!()
    }

    fn get_event_type(&self, name: &str) -> script::Result<EventType> {
        unimplemented!()
    }

    fn register_event_handler(&mut self, event_type: EventType, handler: script::EventHandler) {
        unimplemented!()
    }
}
