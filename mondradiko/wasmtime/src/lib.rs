use mondradiko_core::{script, EventType};
use script::Api;

pub struct Instance<T> {
    store: wasmtime::Store<T>,
    instance: wasmtime::Instance,
}

impl<T: Api> Instance<T> {
    pub fn new(api: T, bytes: &[u8]) -> anyhow::Result<Self> {
        let engine = wasmtime::Engine::default();
        let linker = wasmtime::Linker::<T>::new(&engine);
        let module = wasmtime::Module::new(&engine, bytes)?;
        let mut store = wasmtime::Store::new(&engine, api);
        let instance = linker.instantiate(&mut store, &module)?;
        Ok(Self { store, instance })
    }
}

impl<T: Api> script::Instance for Instance<T> {
    fn handle_event(&mut self, _type: EventType, _data: &[u8]) {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use script::MockApi;

    #[test]
    fn instance_init() -> anyhow::Result<()> {
        let wat = b"(module)";
        let _instance = Instance::new(MockApi::new(), wat)?;
        Ok(())
    }
}
