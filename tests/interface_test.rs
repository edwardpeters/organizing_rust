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

#[test]
fn test_toy_car(){
    let toy_car = ToyCar::new(Paint::Green);
    assert_eq!(format!("{}", toy_car), "Toy Car with a A Car Engine, a Green, a a Chassis, but smaller and more toy-like, and a Window.");
}