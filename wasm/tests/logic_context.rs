use rask_wasm::logic::LogicContext;

#[test]
fn create() {
    let ctx = LogicContext::new();
    println!("Context {:?}", ctx);
}
