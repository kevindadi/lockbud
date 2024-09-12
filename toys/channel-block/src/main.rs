use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::sync_channel(1); // 有界通道，容量为 1

    let handle = thread::spawn(move || {
        // 发送第一个消息
        tx.send(1).unwrap();
        println!("Sent 1");

        // 因为通道已满，下面的发送操作会阻塞，直到接收端接收消息
        tx.send(2).unwrap();
        println!("Sent 2");
    });

    // 模拟延迟接收，导致发送者阻塞
    thread::sleep(Duration::from_secs(2));
    println!("Received: {}", rx.recv().unwrap()); // 读取第一个消息

    handle.join().unwrap();
}
