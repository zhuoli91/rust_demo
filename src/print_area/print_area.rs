struct Circle {
    radius: f32,
}

struct Rectangle {
    width: f32,
    height: f32,
}

trait Area {
    fn area(&self) -> f32;
}

impl Area for Circle {
    fn area(&self) -> f32 {
        use std::f32::consts::PI;
        PI * self.radius.powf(2.0)
    }
}

impl Area for Rectangle {
    fn area(&self) -> f32 {
        self.width * self.height
    }
}

fn print_area<T: Area>(t: &T) {
    println!("area is {}", t.area())
}

pub fn run() {
    let circle = Circle { radius: 5.0 };
    print_area(&circle);

    let rectangle = Rectangle {
        width: 10.0,
        height: 20.0,
    };
    print_area(&rectangle);
}
