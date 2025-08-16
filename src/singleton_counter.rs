use std::sync::{LazyLock, Mutex};

pub struct SingletonCounter {
    count: usize,
}
impl SingletonCounter {
    pub fn count(&self) -> usize {
        self.count
    }

    pub fn get() -> &'static Mutex<SingletonCounter> {
        static INSTANCE: LazyLock<Mutex<SingletonCounter>> = LazyLock::new(|| {
            Mutex::new(
                SingletonCounter { count: 0 }
            )
        });
        &INSTANCE
    }

    pub fn add(&mut self, num: usize) {
        self.count += num;
    }
}
