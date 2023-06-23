use std::{thread, time::Duration};

use flume::{Receiver, Sender};
use zbus::blocking::Connection;

fn run_message_generator(sender: Sender<i32>) {
    for i in 0..i32::MAX {
        thread::sleep(Duration::from_secs(1));
        sender.send(i).unwrap();
    }
}

fn main() {
    let (tx, rx) = flume::unbounded();

    thread::spawn(|| run_message_generator(tx));
    send_signals(rx);
}

fn send_signals(receiver: Receiver<i32>) {
    let connection = Connection::session().unwrap();

    loop {
        let msg_number = receiver.recv().unwrap();
        let msg_body = format!("climsg-msg-body-{msg_number}");

        println!("Sending message {msg_body}");

        let result = connection
            .emit_signal(
                Some("climsg.client.listener"),
                "/climsg_object_path",
                "climsg.interface",
                "climsg_signal_name",
                &msg_body,
            )
            .expect("Failed to send message");

        dbg!(result);
    }
}
