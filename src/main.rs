use mondradiko_core::*;
use mondradiko_types::*;
use std::sync::Arc;

struct ExampleInstance<T> {
    api: T,
}

impl<T: ScriptApi> ScriptInstance for ExampleInstance<T> {
    fn handle_event(&mut self, _event_type: EventType, _data: &[u8]) {
        let component_id = self.api.get_component_id("position").unwrap();
        let position = components::Position { position: [0.0; 3] };
        let positions = &[position];
        let components: &[u8] = bytemuck::cast_slice(positions);
        let entities = &[EntityId(0)];
        self.api
            .write_components(component_id, entities, components);
    }
}

fn main() {
    println!("Hello, world!");
    let core = Arc::new(Core::default());

    // TODO add wasmtime script instance
    let script = ExampleInstance {
        api: BasicCoreApi::new(core.clone()),
    };
    core.add_script(Box::new(script));

    for _ in 0..10 {
        core.step();
    }
}
