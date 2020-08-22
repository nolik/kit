extern crate clap;

use std::time::Duration;

use clap::{App, Arg};
use futures::StreamExt;
use log::{info, warn, LevelFilter, SetLoggerError};
use rdkafka::client::ClientContext;
use rdkafka::config::{ClientConfig, RDKafkaLogLevel};
use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::consumer::{CommitMode, Consumer, ConsumerContext, Rebalance};
use rdkafka::error::KafkaResult;
use rdkafka::message::{Headers, Message, OwnedHeaders};
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::topic_partition_list::TopicPartitionList;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let matches = App::new("kcli")
        .version("1.0")
        .about("Kafka cli helper")
        .arg(
            Arg::with_name("broker")
                .short("b")
                .long("brokers")
                .help("Sets kafka broker ip with port")
                .takes_value(true)
                .default_value("localhost:9092"),
        )
        .arg(
            Arg::with_name("consumer")
                .short("c")
                .help("consume msg's from topic (eager by default, 'test default topic)")
                .takes_value(true)
                .default_value("test"),
        )
        .arg(
            Arg::with_name("producer")
                .short("p")
                .help("produce msg's to topic")
                .takes_value(true)
                .default_value("test"),
        )
        .get_matches();

    //    CLI logic
    let broker = matches.value_of("broker").unwrap();
    let rand_group_id = Uuid::new_v4().to_string() + "_kcli";
    let topics = matches.values_of("topic").unwrap().collect::<Vec<&str>>();

    consume_and_print(broker, rand_group_id.as_ref(), &topics).await
}

// A context can be used to change the behavior of producers and consumers by adding callbacks
// that will be executed by librdkafka.
// This particular context sets up custom callbacks to log rebalancing events.
struct CustomContext;

impl ClientContext for CustomContext {}

impl ConsumerContext for CustomContext {
    fn pre_rebalance(&self, rebalance: &Rebalance) {
        info!("Pre rebalance {:?}", rebalance);
    }

    fn post_rebalance(&self, rebalance: &Rebalance) {
        info!("Post rebalance {:?}", rebalance);
    }

    fn commit_callback(&self, result: KafkaResult<()>, _offsets: &TopicPartitionList) {
        info!("Committing offsets: {:?}", result);
    }
}

// A type alias with your custom consumer can be created for convenience.
type LoggingConsumer = StreamConsumer<CustomContext>;

async fn consume_and_print(brokers: &str, group_id: &str, topics: &[&str]) {
    let context = CustomContext;

    let consumer: LoggingConsumer = ClientConfig::new()
        .set("group.id", group_id)
        .set("bootstrap.servers", brokers)
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "true")
        //.set("statistics.interval.ms", "30000")
        //.set("auto.offset.reset", "smallest")
        .set_log_level(RDKafkaLogLevel::Debug)
        .create_with_context(context)
        .expect("Consumer creation failed");

    consumer
        .subscribe(&topics.to_vec())
        .expect("Can't subscribe to specified topics");

    // consumer.start() returns a stream. The stream can be used ot chain together expensive steps,
    // such as complex computations on a thread pool or asynchronous IO.
    let mut message_stream = consumer.start();

    while let Some(message) = message_stream.next().await {
        match message {
            Err(e) => warn!("Kafka error: {}", e),
            Ok(m) => {
                let payload = match m.payload_view::<str>() {
                    None => "",
                    Some(Ok(s)) => s,
                    Some(Err(e)) => {
                        warn!("Error while deserializing message payload: {:?}", e);
                        ""
                    }
                };
                info!("key: '{:?}', payload: '{}', topic: {}, partition: {}, offset: {}, timestamp: {:?}",
                      m.key(), payload, m.topic(), m.partition(), m.offset(), m.timestamp());
                if let Some(headers) = m.headers() {
                    for i in 0..headers.count() {
                        let header = headers.get(i).unwrap();
                        info!("  Header {:#?}: {:?}", header.0, header.1);
                    }
                }
                consumer.commit_message(&m, CommitMode::Async).unwrap();
            }
        };
    }

    async fn produce(brokers: &str, topic_name: &str) {
        let producer: &FutureProducer = &ClientConfig::new()
            .set("bootstrap.servers", brokers)
            .set("message.timeout.ms", "5000")
            .create()
            .expect("Producer creation error");

        // This loop is non blocking: all messages will be sent one after the other, without waiting
        // for the results.
        let futures = (0..5)
            .map(|i| async move {
                // The send operation on the topic returns a future, which will be
                // completed once the result or failure from Kafka is received.
                let delivery_status = producer
                    .send(
                        FutureRecord::to(topic_name)
                            .payload(&format!("Message {}", i))
                            .key(&format!("Key {}", i))
                            .headers(OwnedHeaders::new().add("header_key", "header_value")),
                        Duration::from_secs(0),
                    )
                    .await;

                // This will be executed when the result is received.
                info!("Delivery status for message {} received", i);
                delivery_status
            })
            .collect::<Vec<_>>();

        // This loop will wait until all delivery statuses have been received.
        for future in futures {
            info!("Future completed. Result: {:?}", future.await);
        }
    }
}
