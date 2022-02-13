extern crate clap;

use clap::{App, Arg};
use log::{debug, info, warn};
use uuid::Uuid;

use rdkafka::client::ClientContext;
use rdkafka::config::{ClientConfig, RDKafkaLogLevel};
use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::consumer::{CommitMode, Consumer, ConsumerContext, Rebalance};
use rdkafka::error::KafkaResult;
use rdkafka::message::Message;
use rdkafka::topic_partition_list::TopicPartitionList;

#[tokio::main]
async fn main() {
    env_logger::init();

    let matches = App::new("kit")
        .version("0.1")
        .about("Kafka intuitive toolkit")
        .arg(
            Arg::new("broker")
                .short('b')
                .long("brokers")
                .help("Sets kafka broker ip with port")
                .takes_value(true)
                .default_value("localhost:9092"),
        )
        .arg(
            Arg::new("consume")
                .short('c')
                .help("consume msg's from topic (eager by default)"),
        )
        .arg(
            Arg::new("produce")
                .short('p')
                .help("produce msg's to topic (eager by default)"),
        )
        .arg(
            Arg::new("topic")
                .short('t')
                .help("default topic name")
                .takes_value(true)
                .default_value("test"),
        )
        .get_matches();

    //    CLI logic
    let broker = matches.value_of("broker").unwrap();
    let rand_group_id = Uuid::new_v4().to_string() + "_kit";
    let topic = matches.value_of("topic").unwrap();
    let producer_mode = matches.is_present("produce");

    if producer_mode {
        warn!("producer mode not yet implemented");
    } else {
        consume_and_print(broker, rand_group_id.as_ref(), &topic).await
    }
}

// A context can be used to change the behavior of producers and consumers by adding callbacks
// that will be executed by librdkafka.
// This particular context sets up custom callbacks to log rebalancing events.
struct CustomContext;

impl ClientContext for CustomContext {}

impl ConsumerContext for CustomContext {
    fn pre_rebalance(&self, rebalance: &Rebalance) {
        debug!("Pre rebalance {:?}", rebalance);
    }

    fn post_rebalance(&self, rebalance: &Rebalance) {
        debug!("Post rebalance {:?}", rebalance);
    }

    fn commit_callback(&self, result: KafkaResult<()>, _offsets: &TopicPartitionList) {
        debug!("Committing offsets: {:?}", result);
    }
}

// A type alias with your custom consumer can be created for convenience.
type LoggingConsumer = StreamConsumer<CustomContext>;

async fn consume_and_print(brokers: &str, group_id: &str, topic: &str) {
    let context = CustomContext;

    let consumer: LoggingConsumer = ClientConfig::new()
        .set("group.id", group_id)
        .set("bootstrap.servers", brokers)
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "true")
        .set_log_level(RDKafkaLogLevel::Debug)
        .create_with_context(context)
        .expect("Consumer creation failed");

    consumer
        .subscribe(&[topic])
        .expect("Can't subscribe to specified topics");

    debug!("Starting to consume from topics {:?}", &topic);
    loop {
        match consumer.recv().await {
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
                debug!("key: '{:?}', payload: '{}', topic: {}, partition: {}, offset: {}, timestamp: {:?}",
                      m.key(), payload, m.topic(), m.partition(), m.offset(), m.timestamp());
                info!("{}", payload);
                consumer.commit_message(&m, CommitMode::Async).unwrap();
            }
        };
    }
}
