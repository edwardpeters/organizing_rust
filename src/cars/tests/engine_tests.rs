crate::test_prefix!();

#[test]
fn test_engine(){
    let engine = Engine;
    assert_eq!(format!("{}", engine), "A Car Engine");
}