extern crate clap;


use clap::{Arg, App, SubCommand};
mod kafka;

fn main() {
    let matches = App::new("Kafka Cli")
                          .version("1.0")
                          .author("Julien Senon <julien.senon@gmail.com>")
                          .about("Rust producer and consumer kafka command line")
                          .arg(Arg::with_name("v")
                               .short("v")
                               .multiple(true)
                               .help("Sets the level of verbosity"))
                          .subcommand(SubCommand::with_name("consumer")
                                    .about("Launch Consumer")
                                    .arg(Arg::with_name("brokers")
                                        .short("b")
                                        .long("brokers")
                                        .help("Broker list in kafka format")
                                        .takes_value(true)
                                        .default_value("localhost:9092"))
                                    .arg(Arg::with_name("topic")
                                        .short("t")
                                        .long("topic")
                                        .help("Destination topic")
                                        .takes_value(true)
                                        .required(true))
                                    .arg(Arg::with_name("group_id")
                                        .short("g")
                                        .long("group")
                                        .help("Group ID")
                                        .takes_value(true)
                                        .required(true))
                                    .arg(Arg::with_name("debug")
                                        .short("d")
                                        .help("print debug information verbosely")))
                          .subcommand(SubCommand::with_name("producer")
                                    .about("Launch Producer")
                                    .arg(Arg::with_name("msg")
                                        .help("Message to sent")
                                        .required(true)
                                        .index(1))
                                    .arg(Arg::with_name("brokers")
                                        .short("b")
                                        .long("brokers")
                                        .help("Broker list in kafka format")
                                        .takes_value(true)
                                        .default_value("localhost:9092"))
                                    .arg(Arg::with_name("topic")
                                        .short("t")
                                        .long("topic")
                                        .help("Destination topic")
                                        .takes_value(true)
                                        .required(true))
                                    .arg(Arg::with_name("debug")
                                        .short("d")
                                        .help("print debug information verbosely")))
                          .get_matches();

    if let Some(matches) = matches.subcommand_matches("producer") {
        let topic = matches.value_of("topic").unwrap();
        println!("topic: {}", topic);
        let brokers = matches.value_of("brokers").unwrap();
        println!("brokers: {}", brokers);
        let msg = matches.value_of("msg").unwrap();
        println!("message: {}", msg);
        kafka::producer::produce(brokers, topic, msg);
    }

    // Vary the output based on how many times the user used the "verbose" flag
    // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
    // match matches.occurrences_of("v") {
    //     0 => println!("No verbose info"),
    //     1 => println!("Some verbose info"),
    //     2 => println!("Tons of verbose info"),
    //     3 | _ => println!("Don't be crazy"),
    // }

    // You can handle information about subcommands by requesting their matches by name
    // (as below), requesting just the name used, or both at the same time
    if let Some(matches) = matches.subcommand_matches("consumer") {
        let topic = matches.value_of("topic").unwrap();
        let brokers = matches.value_of("brokers").unwrap();
        let group_id = matches.value_of("group_id").unwrap();
        kafka::consumer::consume(brokers, topic, group_id);

    }


}