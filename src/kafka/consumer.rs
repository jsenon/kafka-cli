extern crate rdkafka;


use rdkafka::config::{ClientConfig,RDKafkaLogLevel};
use rdkafka::consumer::stream_consumer::{StreamConsumer};
use rdkafka::consumer::{Consumer,CommitMode};
use rdkafka::message::{Message, Headers};

use futures::stream::Stream;



pub fn consume(brokers: &str, topics: &str, group_id: &str) {

     let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", group_id)
        .set("bootstrap.servers", brokers)
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "false")
        //.set("statistics.interval.ms", "30000")
        //.set("auto.offset.reset", "smallest")
        .set_log_level(RDKafkaLogLevel::Debug)
        .create()
        .expect("Consumer creation failed");


    consumer.subscribe(&[topics]).expect("Can't subscribe to specified topic");

    // consumer.start() returns a stream. The stream can be used ot chain together expensive steps,
    // such as complex computations on a thread pool or asynchronous IO.
    let message_stream = consumer.start();
       
    

    for message in message_stream.wait() {
        match message {
            Err(_) => println!("Error while reading from stream."),
            Ok(Err(e)) => println!("Kafka error: {}", e),
            Ok(Ok(m)) => {
                let payload = match m.payload_view::<str>() {
                    None => "",
                    Some(Ok(s)) => s,
                    Some(Err(e)) => {
                        println!("Error while deserializing message payload: {:?}", e);
                        ""
                    },
                };
                println!("key: '{:?}', payload: '{}', topic: {}, partition: {}, offset: {}, timestamp: {:?}",
                      m.key(), payload, m.topic(), m.partition(), m.offset(), m.timestamp());
                if let Some(headers) = m.headers() {
                    for i in 0..headers.count() {
                        let header = headers.get(i).unwrap();
                        println!("  Header {:#?}: {:?}", header.0, header.1);
                    }
                }
                consumer.commit_message(&m, CommitMode::Async).unwrap();
            },
        };
    }

}
