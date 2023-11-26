mod error;
mod device_model;

use std::{time::Duration, collections::HashMap};
use device_model::DeviceModel;
use paho_mqtt::{AsyncClient, Message, ssl_options};
use dotenv::dotenv;
use serde_json::Value;
use error::MqttError;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;

use crate::device_model::DeviceData;

const TOPICS: &[&str] = &["device/+"];
const TOS: &[i32] = &[1, 2];

const HOST: &str = "wss://lotrando.fit.vutbr.cz:443/mqtt";
const USERNAME: &str = "student";
const PASSWORD: &str = "iV4yyBV2edM3nWS7veUI8rfIJ4nfK2wNB1sdSA5y";

pub struct App {
    pool: Pool<Postgres>,
    mqtt: AsyncClient,
    message_queue: Arc<Mutex<VecDeque<Message>>>
}

impl App {

    pub async fn new() -> Self{
        Self {
            pool: App::init_db_pool().await,
            mqtt: App::init_mqtt().await,
            message_queue: Arc::new(Mutex::new(VecDeque::new()))
        }

    }

    async fn init_db_pool() -> Pool<Postgres> {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        match PgPoolOptions::new()
            .max_connections(10)
            .connect(&database_url)
            .await
        {
            Ok(pool) => {
                println!("✅ Connection to the database is successful!");
                return pool
            }
            Err(err) => {
                println!("🔥 Failed to connect to the database: {:?}", err);
                std::process::exit(1);
            }
        };
    }

    async fn init_mqtt() -> AsyncClient {
        let create_opts = paho_mqtt::CreateOptionsBuilder::new_v3()
            .server_uri(HOST)
            .client_id("dashboard_service")
            .finalize();

        paho_mqtt::AsyncClient::new(create_opts).unwrap()
    }

    fn mqtt_connect(&self) {
        println!("Dasboard service connecting to MQTT server {}", HOST);
        let queue = self.message_queue.clone();
        self.mqtt.set_message_callback(move |cli, message| on_mqtt_message(cli, message, queue.clone()));
        self.mqtt.set_connected_callback(|_|{
            println!("✅ Successfully connected to MQTT server");
        });

        self.mqtt.set_connection_lost_callback(|cli|{
            println!("Lost connection to MQTT server");
            cli.reconnect();
        });

        self.mqtt.set_disconnected_callback(|cli, _, _|{
            println!("Disconnected from MQTT server");
            cli.reconnect();
        });
        

        let conn_opts = paho_mqtt::ConnectOptionsBuilder::new_ws()
            .user_name(USERNAME)
            .password(PASSWORD)
            .ssl_options(ssl_options::SslOptionsBuilder::new()
                .enable_server_cert_auth(false)
                .finalize())
            .keep_alive_interval(Duration::from_secs(5))
            .finalize();

        if let Err(err) = self.mqtt.connect(conn_opts).wait() {
            eprintln!("Unable to connect: {}", err);
            std::process::exit(1);
        }

        self.mqtt.subscribe_many(TOPICS, TOS);
    }
}

#[tokio::main]
async fn main() {
    env_logger::init();
    dotenv().ok();

    let app = App::new().await;
    app.mqtt_connect();

    loop {
        let queue = app.message_queue.clone();

        if queue.lock().unwrap().is_empty() {
            // Saving cpu
            std::thread::sleep(Duration::from_secs(1));
            continue;
        }

        let message = queue.lock().unwrap().pop_front().unwrap();
        let devices = DeviceModel::all_by_topic(&app.pool, message.topic().to_string()).await;

        if devices.is_err() {
            continue;
        }
        let devices = devices.unwrap();
        let map = json_to_map(&message.payload_str());

        if map.is_err() {
            continue;
        }

        let map = map.unwrap();

        for device in devices {
            for structure in device.structure {
                if let Some(value) = map.get(&structure.real_name) {
                    let timestamp = chrono::Utc::now();
                    println!("[{}] Saving {} - {} for device {}({})", timestamp.to_string(), structure.real_name, value.to_string(), device.name, device.device_id);
                    let data = DeviceData {
                        device_id: device.device_id,
                        devicestructure_id: structure.devicestructure_id,
                        value: value.to_string(),
                        timestamp: timestamp,
                    };

                    let result = data.insert(&app.pool).await;
                    if let Err(e) = result {
                        println!("[{}] {}", timestamp.to_string(), e.get_error());
                    }
                }
            }
        }


    }

}

fn json_to_map(data: &str) -> Result<HashMap<String, Value>, MqttError> {
    serde_json::from_str(data).map_err(|e| MqttError::SerdeJsonError(e))
}

pub fn on_mqtt_message(_: &AsyncClient, message: Option<Message>, queue: Arc<Mutex<VecDeque<Message>>>) {
    if let Some(message) = message {
        queue.lock().unwrap().push_back(message);
    }
    else {
        println!("Empty message, ignoring");
    }
    
}