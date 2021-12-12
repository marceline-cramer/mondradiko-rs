use mondradiko_types::*;

pub struct EventType(u32);
pub struct ResourceId(u32);

pub type ScriptError = ();
pub type ScriptResult<T> = Result<T, ScriptError>;

pub type EventHandler = Box<dyn FnMut(&[u8])>;

pub trait ScriptApi {
    /// Loads an asset
    fn load_asset(&mut self, asset_type: AssetType, id: AssetId, data: &[u8]) -> ScriptResult<AssetId>;

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
