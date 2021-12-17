use super::*;

// TODO research #[derive(Error)] to make this work
pub type Error = ();

pub type Result<T> = std::result::Result<T, Error>;

// TODO relocate to dedicated events module
pub type EventHandler = Box<dyn FnMut(&[u8])>;

pub trait Api {
    fn get_asset_type(&self, name: &str) -> Result<AssetType>;
    fn load_asset(&mut self, asset_type: AssetType, data: &[u8]) -> Result<AssetId>;
    fn get_component_id(&self, name: &str) -> Result<ComponentId>;
    fn write_components(&mut self, component: ComponentId, entities: &[EntityId], data: &[u8]);
    fn delete_components(&mut self, component: ComponentId, entities: &[EntityId]);
    fn get_resource_id(&self, name: &str) -> Result<ResourceId>;
    fn write_resource(&mut self, id: ResourceId, data: &[u8]);
    fn get_event_type(&self, name: &str) -> Result<EventType>;
    fn register_event_handler(&mut self, event_type: EventType, handler: EventHandler);
}

pub trait Instance {
    fn handle_event(&mut self, event_type: EventType, data: &[u8]);
}
