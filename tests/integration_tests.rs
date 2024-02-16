#[cfg(test)]
mod integration_tests {
    use rocket::local::asynchronous::Client;
    use rocket::http::{ContentType, Status};
    use rocket::serde::json::json;
    use rust_rest::endpoints::description::{API_DESCRIPTION_PATH, API_DESCRIPTION_RESPONSE};
    use rust_rest::endpoints::hello::{HELLO_PATH, HELLO_RESPONSE};
    use rust_rest::endpoints::thruster::THRUSTER_PATH;

    async fn launch_test_instance() -> Client {
        // Here, we'd dynamically generate or specify a test database URL.
        // For demonstration, using a static test database name.
        let database_url = "sqlite:./rocket_database_test.sqlite";
        let rocket = rust_rest::rocket_with_custom_db(database_url);
        Client::tracked(rocket).await.expect("valid rocket instance")
    }

    #[rocket::async_test]
    async fn test_get_endpoints() {
        let client = launch_test_instance().await;

        let get_endpoint_expectations: Vec<(&str, &str)> = vec![
            (HELLO_PATH, HELLO_RESPONSE),
            (API_DESCRIPTION_PATH, API_DESCRIPTION_RESPONSE),
        ];

        for (path, expected_response) in get_endpoint_expectations {
            let actual_response = client.get(path).dispatch().await;
            assert_eq!(actual_response.status(), Status::Ok);
            assert_eq!(actual_response.into_string().await.unwrap(), expected_response);
        }
    }

    #[rocket::async_test]
    async fn test_post_thruster_endpoint() {
        let client = launch_test_instance().await;
        let data = json!({
            "name": "Cold gas thruster",
            "manufacturer": "Test Manufacturer", // Include this field
            "min_consumption_in_liter_per_second": 12.5,
            "max_consumption_in_liter_per_second": 15.0, // Assume this is also required
            "fuel_type": "Test Fuel" // Include this field if it's required
        });

        let response = client.post(THRUSTER_PATH)
            .header(ContentType::JSON)
            .body(data.to_string())
            .dispatch().await;

        assert_eq!(response.status(), Status::Ok);
        // Adjust the expected response based on your actual application logic
    }
}