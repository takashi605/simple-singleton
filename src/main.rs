use std::sync::{LazyLock, Mutex};

fn main() {
    println!("これはシングルトンパターンの学習用プロジェクトです。テストを実行して、シングルトンの動作を確認してください。");
}

struct SingletonCounter {
    count: usize,
}
impl SingletonCounter {
    fn count(&self) -> usize {
        self.count
    }

    fn get() -> &'static Mutex<SingletonCounter> {
        static INSTANCE: LazyLock<Mutex<SingletonCounter>> = LazyLock::new(|| {
            Mutex::new(
                SingletonCounter { count: 0 }
            )
        });
        &INSTANCE
    }

    fn add(&mut self, num: usize) {
        self.count = num;
    }
}

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
}
