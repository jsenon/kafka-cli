extern crate rdkafka;


use rdkafka::config::ClientConfig;
use rdkafka::producer::{BaseProducer, BaseRecord};

use std::time::Duration;


pub fn produce(brokers: &str, topic_name: &str) {
    println!("Preparing sending message");
    let producer: BaseProducer = ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .set("produce.offset.report", "true")
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation error");
   

    producer.send(
        BaseRecord::to(topic_name)
            .payload("kikoo")
            .key("poduce_cli"),
    ).expect("Failed to enqueue");
    println!("Message send");
    // Poll at regular intervals to process all the asynchronous delivery events.
    for _ in 0..10 {
        producer.poll(Duration::from_millis(100));
    }

    // And/or flush the producer before dropping it.
    producer.flush(Duration::from_secs(1));
}