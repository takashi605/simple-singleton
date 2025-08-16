use simple_singleton::SingletonCounter;

#[test]
fn counters_are_pointing_to_the_same_instance() {
    // SingletonCounter を2回取得して、同じインスタンスを指していることを確認
    let counter1 = SingletonCounter::get();
    let counter2 = SingletonCounter::get();

    assert!(std::ptr::eq(counter1, counter2));
}

#[test]
fn count_state_is_shared() {
    {
        let mut counter = SingletonCounter::get().lock().unwrap();

        // SingletonCounter を取得して、初期状態の count が 0 であることを確認
        assert_eq!(counter.count, 0);

        // SingletonCounter の count を 1 増やす
        counter.add(1);
        assert_eq!(counter.count, 1);
    }

    // 再度 SingletonCounter を取得して、count が 1 であることを確認
    {
        let counter = SingletonCounter::get().lock().unwrap();
        assert_eq!(counter.count, 1);
    }
}
