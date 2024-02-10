crate::test_prefix!();

#[test]
fn test_car(){
    let car = Car::new(Paint::Red); //This test sees "Paint" despite it not being in the car module.
    assert_eq!(format!("{}", car), "Car with 0 windows, a Chassis, a A Car Engine, and Red paint.");
}