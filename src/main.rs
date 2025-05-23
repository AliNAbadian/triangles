// // fn main() {
// //     let width1 = 30;
// //     let height1 = 50;

// //     println!(
// //         "The area of the rectangle is {} square pixels.",
// //         area(width1, height1)
// //     );
// // }

// // fn area(width: u32, height: u32) -> u32 {
// //     width * height
// // }

// // fn main() {
// //     let rect1 = (30, 50);

// //     println!(
// //         "The area of the rectangle is {} square pixels.",
// //         area(rect1)
// //     );
// // }

// // fn area(dimensions: (u32, u32)) -> u32 {
// //     dimensions.0 * dimensions.1
// // }
// #[derive(Debug)]
// struct Rectangle {
//     weight: u64,
//     height: u64,
// }

// fn main() {
//     let rect1 = Rectangle {
//         weight: 30,
//         height: 60,
//     };

//     println!("The are of rectangle is {rect1:?}")
// }

// fn area(rectangle: Rectangle) -> u64 {
//     return rectangle.height * rectangle.weight;
// }

// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         rect1.area()
//     );
// }

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    let sq = Rectangle::square(30);

    println!("This is sq {}", sq.height);
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
