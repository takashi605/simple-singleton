use simple_singleton::SingletonCounter;

#[test]
fn test_count_state_is_shared() {
    {
        let counter = SingletonCounter::get().lock().unwrap();
        let count = counter.count;
        assert_eq!(count, 0);
    }

    {
        let mut counter = SingletonCounter::get().lock().unwrap();
        counter.add(1);
        let count = counter.count;
        assert_eq!(count, 1);
    }

    {
        let counter = SingletonCounter::get().lock().unwrap();
        let count = counter.count;
        assert_eq!(count, 1);
    }
}

#[test]
fn test_counter_is_singleton() {
    let counter1 = SingletonCounter::get();
    let counter2 = SingletonCounter::get();
    assert!(std::ptr::eq(counter1, counter2));
}