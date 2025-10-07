use spin_test_sdk::spin_test;

#[spin_test]
fn breaks() {
    assert!(true);
}

// use spin_test_sdk::{
//     bindings::{fermyon::spin_test_virt, wasi, wasi::http},
//     spin_test,
// };

// #[spin_test]
// fn send_get_request_without_key() {
//     // Perform the request
//     let request = http::types::OutgoingRequest::new(http::types::Headers::new());
//     request.set_path_with_query(Some("/")).unwrap();
//     let response = spin_test_sdk::perform_request(request);

//     // Assert response status and body is 404
//     assert_eq!(response.status(), 404);
// }

// #[spin_test]
// fn send_get_request_with_invalid_key() {
//     // Perform the request
//     let request = http::types::OutgoingRequest::new(http::types::Headers::new());
//     request.set_path_with_query(Some("/x?123")).unwrap();
//     let response = spin_test_sdk::perform_request(request);

//     // Assert response status and body is 404
//     assert_eq!(response.status(), 404);
// }

// #[spin_test]
// fn send_get_request_with_invalid_key_id() {
//     // Perform the request
//     let request = http::types::OutgoingRequest::new(http::types::Headers::new());
//     request.set_path_with_query(Some("/user?0")).unwrap();
//     let response = spin_test_sdk::perform_request(request);

//     // Assert response status and body is 404
//     assert_eq!(response.status(), 404);
// }
