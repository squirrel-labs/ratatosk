use double_buffer::DoubleBuffer;

#[test]
fn test_initial_empty() {
    let mut db: DoubleBuffer<u32> = DoubleBuffer::new();

    assert!(db.borrow_reader().is_none());
}
