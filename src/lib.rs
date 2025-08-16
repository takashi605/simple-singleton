mod singleton_counter;

use singleton_counter::SingletonCounter;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_state_is_shared() {
        {
            let counter = SingletonCounter::get().lock().unwrap();
            let count = counter.count();
            assert_eq!(count, 0);
        }

        {
            let mut counter = SingletonCounter::get().lock().unwrap();
            counter.add(1);
            let count = counter.count();
            assert_eq!(count, 1);
        }

        {
            let counter = SingletonCounter::get().lock().unwrap();
            let count = counter.count();
            assert_eq!(count, 1);
        }
    }

    #[test]
    fn test_counter_is_singleton() {
        let counter1 = SingletonCounter::get();
        let counter2 = SingletonCounter::get();
        assert!(std::ptr::eq(counter1, counter2));
    }

    // TODO doctest にする
    // fn test_counter_is_not_generate_to_literals() {
    //     let _counter = SingletonCounter { count: 0, _private: Private };
    // }
}
