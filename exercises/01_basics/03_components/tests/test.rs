extern crate components;

unsafe extern "C" {
    fn add_random_u64(num: u64) -> u64;
}

#[test]
fn verify_it_works() {
    assert!(unsafe { add_random_u64(10) } >= 10);
}
