use simple_singleton::SingletonCounter;

#[test]
fn counters_are_pointing_to_the_same_instance() {
    let counter1 = SingletonCounter::get();
    let counter2 = SingletonCounter::get();
    assert!(std::ptr::eq(counter1, counter2));
}

#[test]
fn count_state_is_shared() {
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
