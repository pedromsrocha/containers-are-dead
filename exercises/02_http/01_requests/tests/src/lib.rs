use spin_test_sdk::{
    bindings::fermyon::spin_wasi_virt::http_handler, bindings::wasi::http, spin_test,
};
use std::str::FromStr;

#[spin_test]
fn breaks() {
    for _ in 0..50 {
        let response = http::types::OutgoingResponse::new(http::types::Headers::new());
        response.write_body("40".as_bytes());
        http_handler::set_response(
            "http://www.randomnumberapi.com/api/v1.0/random?min=30&max=50",
            http_handler::ResponseHandler::Response(response),
        );

        // Perform the request
        let request = http::types::OutgoingRequest::new(http::types::Headers::new());
        request
            .set_path_with_query(Some("/?min=30&max=50"))
            .unwrap();
        let response = spin_test_sdk::perform_request(request);
        let response = response.body().unwrap();
        let response = str::from_utf8(&response).unwrap().trim();
        let response = f64::from_str(response).unwrap();

        assert!(response >= 30.0 && response <= 50.0);
    }
}
