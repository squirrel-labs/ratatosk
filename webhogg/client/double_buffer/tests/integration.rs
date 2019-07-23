use double_buffer::DoubleBuffer;

#[test]
fn test_initial_empty() {
    let mut db: DoubleBuffer<u32> = DoubleBuffer::new();

    assert!(db.borrow_reader().is_none());
}

#[test]
fn test_simple_read_write() {
    let mut db = DoubleBuffer::new();
    {
        let mut writer = db.borrow_writer();
        writer.set(42);
    }
    {
        let reader = db.borrow_reader().unwrap();
        assert_eq!(*reader.get(), 42);
    }
}
