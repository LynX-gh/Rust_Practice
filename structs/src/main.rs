// An attribute to hide warnings for unused code.
#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
#[derive(Debug)]
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,     // (x1, y1)
    bottom_right: Point, // (x2, y2)
}

impl Rectangle {
    fn area(&self) -> f32 {
        let Rectangle {
            top_left: Point { x: x1, y: y1 },
            bottom_right: Point { x: x2, y: y2 },
        } = self;
        // let Point { x: x1, y: y1 } = self.top_left;
        // let Point { x: x2, y: y2 } = self.bottom_right;
        ((x2 - x1) * (y2 - y1)).abs()
    }
}

fn square(p1: Point, dist: f32) -> Rectangle {
    Rectangle {
        top_left: Point { ..p1 },
        bottom_right: Point {
            x: p1.x + dist,
            y: p1.y + dist,
        },
    }
}

fn main() {
    // Create struct with field init shorthand
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };
    let nnm = Person {
        name: String::from("Neil\n Nitin\n Mukesh"),
        age: 40,
    };

    // Print debug struct
    println!("{:?}", peter);
    println!("{:?}", nnm);
    // println!("{}", nnm.name);

    // Instantiate a `Point`
    let point: Point = Point { x: 2.00, y: 2.00 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right = Point { x: 4.00, y: 4.00 };

    // `bottom_right.y` will be the same as `point.y` because we used that field
    // from `point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    let Point {
        x: left_edge,
        y: top_edge,
    } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right,
    };
    println!("{:#?}", _rectangle);
    println!("Area of Rectangle is {}", _rectangle.area());
    println!("Square from rectangle is {:?}", square(point, 69f32));

    // Instantiate a unit struct
    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);
}
