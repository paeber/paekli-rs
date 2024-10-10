fn main() {
    panic!("Paekli LLC is currentli closed 😢");
}

#[test]
#[should_panic]
fn paekli_llc_is_closed() {
    main();
}