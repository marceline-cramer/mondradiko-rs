use components::PositionComponent;
use mondradiko_types::*;
use std::cell::RefCell;
use std::sync::Arc;

#[derive(Clone, Copy)]
pub struct EventType(u32);

pub struct ResourceId(u32);

pub type ScriptError = ();
pub type ScriptResult<T> = Result<T, ScriptError>;

pub type EventHandler = Box<dyn FnMut(&[u8])>;

pub trait ScriptApi {
    /// Loads an asset
    fn load_asset(
        &mut self,
        asset_type: AssetType,
        id: AssetId,
        data: &[u8],
    ) -> ScriptResult<AssetId>;

    /// Retrieves an asset type by name
    fn get_asset_type(&self, name: &str) -> ScriptResult<AssetType>;

    /// Writes components
    fn write_components(&mut self, component: ComponentId, entities: &[EntityId], data: &[u8]);

    /// Deletes components
    fn delete_components(&mut self, component: ComponentId, entities: &[EntityId]);

    /// Retrieves a resource ID by name
    fn get_resource_id(&self, name: &str) -> ScriptResult<ResourceId>;

    /// Writes a resource
    fn write_resource(&mut self, id: ResourceId, data: &[u8]);

    /// Retrieves an event type by name
    fn get_event_type(&self, name: &str) -> ScriptResult<EventType>;

    /// Registers an event handler
    fn register_event_handler(&mut self, event_type: EventType, handler: EventHandler);
}

pub trait ScriptInstance {
    fn handle_event(&mut self, event_type: EventType, data: &[u8]);
}

#[derive(Default)]
pub struct Core {
    scripts: RefCell<Vec<Box<dyn ScriptInstance>>>,
    positions: RefCell<Vec<PositionComponent>>,
}

impl Core {
    pub fn add_script(&self, script: Box<dyn ScriptInstance>) {
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

        if self.positions.borrow().len() > 0 {
            println!("Positions:");
            for position in self.positions.borrow().iter() {
                println!("  {:?}", position.position);
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
}

impl ScriptApi for BasicCoreApi {
    fn load_asset(
        &mut self,
        asset_type: AssetType,
        id: AssetId,
        data: &[u8],
    ) -> ScriptResult<AssetId> {
        unimplemented!()
    }

    fn get_asset_type(&self, name: &str) -> ScriptResult<AssetType> {
        unimplemented!()
    }

    fn write_components(&mut self, component: ComponentId, entities: &[EntityId], data: &[u8]) {
        // TODO implement component registry
        assert_eq!(component.0, 0);

        // TODO actually insert by entity id
        // TODO cast to different component types
        let positions: &[PositionComponent] = bytemuck::cast_slice(data);
        self.core
            .positions
            .borrow_mut()
            .extend_from_slice(positions);
    }

    fn delete_components(&mut self, component: ComponentId, entities: &[EntityId]) {
        unimplemented!()
    }

    fn get_resource_id(&self, name: &str) -> ScriptResult<ResourceId> {
        unimplemented!()
    }

    fn write_resource(&mut self, id: ResourceId, data: &[u8]) {
        unimplemented!()
    }

    fn get_event_type(&self, name: &str) -> ScriptResult<EventType> {
        unimplemented!()
    }

    fn register_event_handler(&mut self, event_type: EventType, handler: EventHandler) {
        unimplemented!()
    }
}
