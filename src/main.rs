#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, Rocket!"
}

#[get("/other")]
fn other() -> &'static str {
    "Other endpoint!"
}

fn get_server() -> rocket::Rocket<rocket::Build> {
    rocket::build().mount("/", routes![index, other])
}

#[launch]
fn boot() -> _ {
    get_server()
}

#[cfg(test)]
mod test {
    use rocket::http::Status;
    use rocket::local::blocking::Client;

    #[test]
    fn test_endpoints() {
        // Launch the Rocket application
        let rocket = super::get_server();
        let client = Client::tracked(rocket).expect("valid rocket instance");

        let get_endpoint_expectations: Vec<(&str, &str)> =
            vec![("/", "Hello, Rocket!"), ("/other", "Other endpoint!")];

        get_endpoint_expectations
            .into_iter()
            .for_each(|(path, expected_response)| {
                let actual_response = client.get(path).dispatch();
                assert_eq!(actual_response.status(), Status::Ok);
                assert_eq!(actual_response.into_string().unwrap(), expected_response);
                println!("Expectations for path '{}' fulfilled", path);
            });
    }
}
