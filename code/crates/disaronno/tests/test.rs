#[cfg(test)]
#[test]
pub fn compiles() {
    let timestamp: disaronno::types::TimeStamp = disaronno::types::LocalTime::now().into();
    println!("{:#?}", timestamp);
    assert_eq!(4, 4)
}