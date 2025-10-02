extern crate setup;

#[test]
fn verify_it_works() {
    extern "C" {
        fn it_works() -> bool;
    }

    assert_eq!(unsafe { it_works() }, true);
}
