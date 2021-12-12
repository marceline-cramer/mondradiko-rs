use mondradiko_core::*;
use std::sync::Arc;

struct ExampleInstance {
    core: Arc<Core>,
}

impl ScriptInstance for ExampleInstance {
    fn handle_event(&mut self, event_type: EventType, data: &[u8]) {
        self.core.test_cb();
    }
}

fn main() {
    println!("Hello, world!");
    let mut core = Arc::new(Core::default());

    let script = ExampleInstance {
        core: core.to_owned(),
    };
    core.add_script(Box::new(script));

    // TODO add wasmtime script instance
    core.step();
}
