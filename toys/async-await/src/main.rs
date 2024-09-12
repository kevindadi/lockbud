use std::sync::{Arc, Mutex};
use tokio::time::{sleep, Duration};

struct SharedState {
    task_a_done: bool,
    task_b_done: bool,
}

async fn task_a(state: Arc<Mutex<SharedState>>) {
    println!("Task A: Waiting for Task B to complete...");

    // Task A 等待 Task B 完成
    loop {
        {
            let state = state.lock().unwrap();
            if state.task_b_done {
                break;
            }
        }
        sleep(Duration::from_millis(100)).await; // 模拟等待
    }

    println!("Task A: Task B completed");
}

async fn task_b(state: Arc<Mutex<SharedState>>) {
    println!("Task B: Waiting for Task A to complete...");

    // Task B 等待 Task A 完成
    loop {
        {
            let state = state.lock().unwrap();
            if state.task_a_done {
                break;
            }
        }
        sleep(Duration::from_millis(100)).await; // 模拟等待
    }

    println!("Task B: Task A completed");
}

#[tokio::main]
async fn main() {
    let state = Arc::new(Mutex::new(SharedState {
        task_a_done: false,
        task_b_done: false,
    }));

    let state_a = Arc::clone(&state);
    let handle_a = tokio::spawn(async move {
        task_a(state_a).await;
    });

    let state_b = Arc::clone(&state);
    let handle_b = tokio::spawn(async move {
        task_b(state_b).await;
    });

    // 设置死锁的条件：Task A 和 Task B 永远不会标记自己完成
    let _ = handle_a.await;
    let _ = handle_b.await;
}
