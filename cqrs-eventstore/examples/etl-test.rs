extern crate cqrs_eventstore;
extern crate cqrs;
extern crate cqrs_data;
extern crate hyper;
extern crate hyper_sync_rustls;
extern crate hyper_native_tls;
extern crate serde;
extern crate uuid;
extern crate env_logger;
#[macro_use] extern crate serde_derive;

use cqrs_data::event::{Source, Store};
use hyper::net::HttpsConnector;
use hyper_sync_rustls::TlsClient;
use hyper_native_tls::NativeTlsClient;

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
struct Data {
    pub winter: String,
    pub is_bool: bool,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
struct Metadata {
    pub who: String,
    pub when: usize,
}

fn main() {
    env_logger::init();

//    let client = hyper::Client::with_connector(HttpsConnector::new(TlsClient::new()));
    let client = hyper::Client::with_connector(HttpsConnector::new(NativeTlsClient::new().unwrap()));
//    let client = hyper::Client::new();
    let conn = cqrs_eventstore::http::EventStoreConnection::new(
        client,
        hyper::Url::parse("https://eventstore.stg-k8s.mswavailability.vpsvc.com/").unwrap(),
//        hyper::Url::parse("http://127.0.0.1:2113/").unwrap(),
        "admin".to_string(),
        "changeit".to_string(),
    );

    let es = cqrs_eventstore::EventStore::<Data, Metadata>::new(&conn);

    let agg_id = ::std::env::var("AGG_ID").unwrap_or("test-5".to_string());

    if false {
        let instant = ::std::time::Instant::now();

        let data = Data {
            winter: "spring".to_string(),
            is_bool: true,
        };
        let metadata = Metadata {
            who: "someone".to_string(),
            when: instant.elapsed().as_secs() as usize,
        };

        let event = cqrs_eventstore::EventEnvelope {
            event_id: uuid::Uuid::new_v4(),
            event_type: "InitialGeneratedEvent".to_string(),
            data,
            metadata: Some(metadata),
        };
        es.append_events(&agg_id, &[event], cqrs_data::Expectation::New)
            .unwrap_or_default();


        for i in 0..100 {
            let data = Data {
                winter: "spring".to_string(),
                is_bool: true,
            };
            let metadata = Metadata {
                who: "someone".to_string(),
                when: instant.elapsed().as_secs() as usize,
            };

            let event = cqrs_eventstore::EventEnvelope {
                event_id: uuid::Uuid::new_v4(),
                event_type: "GeneratedEvent".to_string(),
                data,
                metadata: Some(metadata),
            };
            es.append_events(&agg_id, &[event], cqrs_data::Expectation::LastEvent(cqrs::EventNumber::new(i)))
                .unwrap_or_default();
//        print!(".");
        }

        println!("Appended 10 in {:?}", instant.elapsed());
    }

    let instant = ::std::time::Instant::now();

    let event_iter = es.read_events(&agg_id, cqrs_data::Since::BeginningOfStream).unwrap().unwrap();

    let mut i = 0;
    for e in event_iter {
//        print!(".");
        assert!(e.is_ok());
//        println!("{:#?}", e);
        i += 1;
    }

    println!("Read {} in {:?}", i, instant.elapsed());
}