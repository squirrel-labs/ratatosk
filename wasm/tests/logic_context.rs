use rask_wasm::logic::LogicContext;

#[test]
fn create() {
    let pool = rayon::ThreadPoolBuilder::new().build().unwrap();
    let ctx = LogicContext::new(pool);
}
