use zbus::blocking::{Connection, MessageIterator};

fn main() {
    let connection = Connection::session().unwrap();
    connection.request_name("climsg.client.listener").unwrap();

    let iter = MessageIterator::from(connection);
    for msg in iter {
        println!("Got message: {}", msg.unwrap());
    }
}
