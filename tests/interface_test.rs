use organizing_rust::*;

#[test]
fn test_car(){
    let car = Car::new(Paint::Red);
    assert_eq!(format!("{}", car), "Car with 0 windows, a Chassis, a A Car Engine, and Red paint.");
}

#[test]
fn test_truck(){
    let truck = Truck::new(Paint::SchoolBusYellow);
    assert_eq!(format!("{}", truck), "Truck with a A Truck Engine and School Bus Yellow paint.");
}
