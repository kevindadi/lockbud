use rayon::prelude::*;
use std::sync::{Arc, Mutex};

pub fn join_handle(data: Arc<Mutex<i32>>) {
    // 持有锁时调用 `Rayon::join`
    rayon::join(
        || {
            let mut guard = data.lock().unwrap(); // 这里尝试再次获取锁
            *guard += 1;
            println!("First task done");
        },
        || {
            let mut guard = data.lock().unwrap(); // 这里也尝试获取锁
            *guard += 1;
            println!("Second task done");
        },
    );
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn par_iter_deadlock() {
        let data = Arc::new(Mutex::new(0));
        let items: Vec<i32> = (0..10).collect();
        let binding = data.clone();
        let _guard = binding.lock().unwrap();

        items.par_iter().for_each(|_| {
            let mut guard = data.lock().unwrap();
            *guard += 1;
            println!("Updated shared data: {}", *guard);
        });
        drop(_guard);
        println!("Main task done");
    }

    fn join_deadlock() {
        let data = Arc::new(Mutex::new(0));
        let lock = data.lock().unwrap();

        // 持有锁时调用 `Rayon::join`
        rayon::join(
            || {
                let mut guard = data.lock().unwrap();
                *guard += 1;
                println!("First task done");
            },
            || {
                let mut guard = data.lock().unwrap();
                *guard += 1;
                println!("Second task done");
            },
        );
    }

    #[test]
    fn for_each_with_args() {
        let lock1 = Arc::new(Mutex::new(()));
        let lock2 = Arc::new(Mutex::new(()));

        let locks = vec![
            (lock1.clone(), lock2.clone()),
            (lock2.clone(), lock1.clone()),
        ];

        // 使用 par_iter 并行遍历多个锁组合
        locks.par_iter().for_each(|(first_lock, second_lock)| {
            let _first = first_lock.lock().unwrap();
            std::thread::sleep(std::time::Duration::from_millis(50)); // 模拟处理时间
            let _second = second_lock.lock().unwrap();
            println!("Got both locks!");
        });

        println!("Main task done");
    }
}
