use std::path::Path;

use rocket::http::{ContentType, Status};
use rocket::local::blocking::Client;
use rocket::serde::json::json;

use rust_rest::endpoints::description::{API_DESCRIPTION_PATH, API_DESCRIPTION_RESPONSE};
use rust_rest::endpoints::hello::{HELLO_PATH, HELLO_RESPONSE};
use rust_rest::endpoints::thruster::THRUSTER_PATH;
use rust_rest::get_server;

#[test]
fn test_get_endpoints() {
    let client = Client::tracked(get_server()).expect("valid rocket instance");

    let get_endpoint_expectations: Vec<(&str, &str)> = vec![
        (HELLO_PATH, HELLO_RESPONSE),
        (API_DESCRIPTION_PATH, API_DESCRIPTION_RESPONSE),
    ];

    for (path, expected_response) in get_endpoint_expectations {
        let actual_response = client.get(path).dispatch();
        assert_eq!(actual_response.status(), Status::Ok);
        assert_eq!(actual_response.into_string().unwrap(), expected_response);
    }
}

#[test]
fn test_post_thruster_endpoint() {
    let client = Client::tracked(get_server()).expect("valid rocket instance");
    let data = json!({
        "name": "Cold gas thruster",
        "consumption_in_liter_per_second": 12.5
    });

    let response = client
        .post(THRUSTER_PATH)
        .header(ContentType::JSON)
        .body(data.to_string())
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(
        response.into_string().unwrap(),
        "Thurster named 'Cold gas thruster' successfully saved"
    );
}
