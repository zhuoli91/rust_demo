enum TrafficLight {
    Red,
    Yellow,
    Green,
}

trait Attibute {
    fn get_duration(&self) -> u8;
}

impl Attibute for TrafficLight {
    fn get_duration(&self) -> u8 {
        match &self {
            TrafficLight::Red => 20,
            TrafficLight::Yellow => 5,
            TrafficLight::Green => 60,
        }
    }
}

pub fn run() {
    let red = TrafficLight::Red;
    println!("red light duration is {}", red.get_duration());

    let yellow = TrafficLight::Yellow;
    println!("yellow light duration is {}", yellow.get_duration());
    
    let green = TrafficLight::Green;
    println!("green light duration is {}", green.get_duration());
}
