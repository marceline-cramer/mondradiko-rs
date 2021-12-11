pub struct AssetId(u32);
pub struct AssetType(u32);
pub struct ComponentId(u32);
pub struct EntityId(u32);
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

    /// Registers an event handler
    fn register_event_handler(&mut self, event_name: &str, handler: EventHandler);
}
