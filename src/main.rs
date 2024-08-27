use amiquip::{Connection, ConsumerMessage, ConsumerOptions, Exchange, Publish, QueueDeclareOptions};


fn main() -> amiquip::Result<()>{
    let mut conn =
        Connection::insecure_open("amqp://guest:guest@localhost:5672")?;

    let channel = conn.open_channel(None)?;

    let exchange = Exchange::direct(&channel);
    let queue = channel.queue_declare(
        "Hello",
        QueueDeclareOptions::default()
    )?;

    let message_to_send = b"Hello, world!";
    exchange.publish(
        Publish::new(
            message_to_send,
            "Hello"
        )
    )?;
    println!("Message '{}' was published.", String::from_utf8_lossy(message_to_send));

    let consumer = queue.consume(ConsumerOptions::default())?;

    for (i, message)
    in consumer.receiver().iter().enumerate() {
        match message {
            ConsumerMessage::Delivery(delivery) => {
                let body = String::from_utf8_lossy(&delivery.body);
                println!("Received message number {} with body {}", i, body);
                consumer.ack(delivery)?;
                println!("Press ctrl-c to exit");
            }
            other=> {
                println!("Consumer ended {:?}", other);
                break;
            }
        }
    }
    conn.close()
}
