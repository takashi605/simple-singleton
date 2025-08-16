use std::sync::{LazyLock, Mutex};

pub struct SingletonCounter {
    count: usize,

    // 外部パッケージからリテラル生成できないようにするためのフィールド
    // (外部から Private 型を参照できないことによる事象)
    _private: Private,
}
struct Private;

impl SingletonCounter {
    pub fn count(&self) -> usize {
        self.count
    }

    pub fn get() -> &'static Mutex<SingletonCounter> {
        static INSTANCE: LazyLock<Mutex<SingletonCounter>> = LazyLock::new(|| {
            Mutex::new(
                SingletonCounter { count: 0, _private: Private }
            )
        });
        &INSTANCE
    }

    pub fn add(&mut self, num: usize) {
        self.count += num;
    }
}
