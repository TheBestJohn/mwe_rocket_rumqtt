#![feature(plugin, decl_macro, proc_macro_hygiene)]
extern crate rumqtt;
#[macro_use] extern crate rocket;
use rumqtt::{MqttClient, MqttOptions, QoS, SecurityOptions, ReconnectOptions};
use rocket::State;


#[get("/")]
fn hello(client_state:State<MqttClient>) -> &'static str {
    let res = client_state.inner().publish(
            "test",
            QoS::AtMostOnce,
            false,
            String::from("This is a test of the server"),
        )
        ;
    match res{
        Ok(_) => "ok check your topic for the payload",
        Err(e) => {
            std::dbg!(e);
            "Something went wrong"
        }
    }
    
}

fn main() {
    //Set up MQTT Client
 let reconnection_options = ReconnectOptions::Always(10);
 let secops = SecurityOptions::UsernamePassword(String::from("mqttTest"), String::from(""));

 let mqtt_options = MqttOptions::new("mqttTest", "127.0.0.1", 1883)
     .set_keep_alive(60)
     .set_security_opts(secops)
     .set_inflight(3)
     .set_request_channel_capacity(3)
     .set_reconnect_opts(reconnection_options)
     .set_clean_session(false);
 let (mqtt_client, _notifications) = MqttClient::start(mqtt_options).unwrap();
 
 rocket::ignite().mount("/", routes![hello]).manage(mqtt_client).launch();

}
