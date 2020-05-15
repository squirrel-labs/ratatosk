use rask_wasm::communication::{message_queue::MessageQueueElement, Message, MessageQueue};
use rask_wasm::graphics::context;
use rask_wasm::logic::LogicContext;

static mut MESSAGES: &mut [MessageQueueElement] = &mut [MessageQueueElement::new()];

#[test]
fn create() {
    let message_queue = MessageQueue::new(unsafe { MESSAGES });
    let ctx = LogicContext::new(message_queue);
    println!("Context {:?}", ctx);
    assert!(1 == 1);
}
