use std::sync::{LazyLock, Mutex};

/// この構造体はシングルトンパターンを実装したカウンターです。
/// 外部から直接リテラル生成ができないようになっています。
/// 
/// ```rust,compile_fail
/// use simple_singleton::SingletonCounter;
/// let _counter = SingletonCounter { count: 0, _private: Private };
/// ```
pub struct SingletonCounter {
    pub count: usize,

    // 外部パッケージからリテラル生成できないようにするためのフィールド
    // (外部から Private 型を参照できないことによる事象)
    _private: Private,
}
struct Private;

impl SingletonCounter {
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
