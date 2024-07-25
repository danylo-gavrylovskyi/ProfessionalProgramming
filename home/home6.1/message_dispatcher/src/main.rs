mod messages;
mod message_dispatcher;

use std::sync::Arc;
use std::thread;
use std::time::Duration;

use crate::message_dispatcher::MessageDispatcher;
use crate::messages::{BlueMessage, GreenMessage, OrangeMessage};

fn generate_messages(dispatcher: Arc<MessageDispatcher>, thread_id: i32) {
    for _ in 0..10 {
        let green_msg = GreenMessage {
            text: format!("GreenMessage from thread {}", thread_id),
            counter: rand::random::<i32>(),
        };
        let blue_msg = BlueMessage {
            value1: rand::random::<f64>(),
            value2: rand::random::<f64>(),
        };
        let orange_msg = OrangeMessage {
            text1: "Orange".to_string(),
            text2: "Message".to_string(),
            integer: rand::random::<i32>(),
            value: rand::random::<f64>(),
        };
        
        dispatcher.publish_green_message(green_msg);
        dispatcher.publish_blue_message(blue_msg);
        dispatcher.publish_orange_message(orange_msg);
        
        thread::sleep(Duration::from_millis(100));
    }
}

fn main() {
    let dispatcher = Arc::new(MessageDispatcher::new());

    let green_sub = dispatcher.subscribe_to_green_message(Arc::new(|msg: &GreenMessage| {
        println!("Received GreenMessage: {}, counter: {}", msg.text, msg.counter);
    }));
    let blue_sub = dispatcher.subscribe_to_blue_message(Arc::new(|msg: &BlueMessage| {
        println!("Received BlueMessage: {}, {}", msg.value1, msg.value2);
    }));
    let orange_sub = dispatcher.subscribe_to_orange_message(Arc::new(|msg: &OrangeMessage| {
        println!("Received OrangeMessage: {}, {}, {}, {}", msg.text1, msg.text2, msg.integer, msg.value);
    }));

    let dispatcher_clone = Arc::clone(&dispatcher);
    let handle1 = thread::spawn(move || generate_messages(dispatcher_clone, 1));

    let dispatcher_clone = Arc::clone(&dispatcher);
    let handle2 = thread::spawn(move || generate_messages(dispatcher_clone, 2));

    handle1.join().unwrap();
    handle2.join().unwrap();

    dispatcher.unsubscribe(green_sub);
    dispatcher.unsubscribe(blue_sub);
    dispatcher.unsubscribe(orange_sub);

    println!("Unsubscribed all handlers.");
}
