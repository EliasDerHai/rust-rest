use rocket::http::{ContentType, Status};
use rocket::local::asynchronous::Client;
use rocket::serde::json::serde_json;
use uuid::Uuid;

use rust_rest::CustomizedRocket;
use rust_rest::endpoints::description::{API_DESCRIPTION_PATH, API_DESCRIPTION_RESPONSE};
use rust_rest::endpoints::hello::{HELLO_PATH, HELLO_RESPONSE};
use rust_rest::endpoints::thruster::{RocketThruster, RocketThrusterEntity, THRUSTER_PATH, THRUSTERS_PATH};

async fn launch_test_instance() -> Client {
    let database_url = format!("sqlite:./rocket_database_test{}.sqlite", Uuid::new_v4());
    let rocket = rocket::custom(rocket::Config::figment()
        .merge(("databases.rocket_database.url", database_url)));
    Client::tracked(rocket.take_off()).await.expect("valid rocket instance")
}

#[rocket::async_test]
async fn string_endpoints() {
    let client = launch_test_instance().await;

    let get_endpoint_expectations: Vec<(&str, &str)> = vec![
        (HELLO_PATH, HELLO_RESPONSE),
        (API_DESCRIPTION_PATH, API_DESCRIPTION_RESPONSE),
    ];

    for (path, expected_response) in get_endpoint_expectations {
        let actual_response = client
            .get(path)
            .dispatch().await;

        assert_eq!(actual_response.status(), Status::Ok);
        assert_eq!(actual_response.into_string().await.unwrap(), expected_response);
    }
}

// thruster

fn get_test_thruster() -> RocketThruster {
    RocketThruster {
        name: "Cold gas thruster".to_string(),
        manufacturer: "Test Manufacturer".to_string(),
        min_consumption_in_liter_per_second: 12.5,
        max_consumption_in_liter_per_second: 15.0,
        fuel_type: "Test Fuel".to_string(),
    }
}

#[rocket::async_test]
async fn post_thruster_endpoint() {
    let client = launch_test_instance().await;
    let test_thruster = get_test_thruster();
    let serialized_thruster = serde_json::to_string(&test_thruster).expect("Failed to serialize thruster");

    let response = client
        .post(THRUSTER_PATH)
        .header(ContentType::JSON)
        .body(serialized_thruster)
        .dispatch().await;

    assert_eq!(response.status(), Status::Ok);

    let actual: RocketThrusterEntity = response.into_json().await.expect("valid JSON response");

    assert!(actual.id > 0, "Expected a valid ID greater than 0");
    assert_eq!(actual.name, test_thruster.name);
    assert_eq!(actual.manufacturer, test_thruster.manufacturer);
    assert_eq!(actual.min_consumption_in_liter_per_second, test_thruster.min_consumption_in_liter_per_second);
    assert_eq!(actual.max_consumption_in_liter_per_second, test_thruster.max_consumption_in_liter_per_second);
    assert_eq!(actual.fuel_type, test_thruster.fuel_type);
}

#[rocket::async_test]
async fn get_thrusters_endpoint() {
    let client = launch_test_instance().await;
    let test_thruster = get_test_thruster();
    let serialized_thruster = serde_json::to_string(&test_thruster).expect("Failed to serialize thruster");

    client
        .post(THRUSTER_PATH)
        .header(ContentType::JSON)
        .body(serialized_thruster)
        .dispatch().await;

    let response = client
        .get(THRUSTERS_PATH)
        .dispatch().await;

    let actual: Vec<RocketThrusterEntity> = response.into_json().await.expect("valid JSON response");

    let expected_response = vec![
        RocketThrusterEntity {
            id: 1,
            name: "Cold gas thruster".to_string(),
            manufacturer: "Test Manufacturer".to_string(),
            min_consumption_in_liter_per_second: 12.5,
            max_consumption_in_liter_per_second: 15.0,
            fuel_type: "Test Fuel".to_string(),
        }
    ];

    assert_eq!(expected_response, actual);
}