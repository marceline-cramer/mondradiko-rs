use mondradiko_core::*;
use mondradiko_types::*;
use std::sync::Arc;

struct ExampleInstance<T> {
    api: T,
    entity: EntityId,
}

impl<T: script::ComponentApi + script::AssetApi> ExampleInstance<T> {
    fn new(index: u32, mut api: T) -> Self {
        let asset_type = api.get_asset_type("label").unwrap();
        let title = format!("Foobar {}", index);
        let label = assets::label::LabelAsset { title };
        let asset_data = bincode::serialize(&label).unwrap();
        let asset_id = api.load_asset(asset_type, &asset_data).unwrap();
        let label = components::Label { label: asset_id };
        let labels = &[label];

        let component_id = api.get_component_id("label").unwrap();
        let components: &[u8] = bytemuck::cast_slice(labels);
        let entity = EntityId(index);
        let entities = &[entity];
        api.write_components(component_id, entities, components);

        Self { api, entity }
    }
}

impl<T: script::ComponentApi + script::AssetApi> script::Instance for ExampleInstance<T> {
    fn handle_event(&mut self, _event_type: EventType, _data: &[u8]) {
        let component_id = self.api.get_component_id("position").unwrap();
        let position = components::Position { position: [0.0; 3] };
        let positions = &[position];
        let components: &[u8] = bytemuck::cast_slice(positions);
        let entities = &[self.entity];
        self.api
            .write_components(component_id, entities, components);
    }
}

fn main() {
    println!("Hello, world!");
    let core = Arc::new(Core::new());

    // TODO add wasmtime script instance
    let script = ExampleInstance::new(0, BasicCoreApi::new(core.clone()));
    core.add_script(Box::new(script));
    let script = ExampleInstance::new(1, BasicCoreApi::new(core.clone()));
    core.add_script(Box::new(script));
    let script = ExampleInstance::new(2, BasicCoreApi::new(core.clone()));
    core.add_script(Box::new(script));

    for _ in 0..10 {
        core.step();
    }
}
