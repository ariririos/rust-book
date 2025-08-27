#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

#[derive(Debug)]
struct PointMultiple<X1, Y1> {
    x: X1,
    y: Y1
}

impl<X1, Y1> PointMultiple<X1, Y1> {
    fn mixup<X2, Y2>(self, other: PointMultiple<X2, Y2>) -> PointMultiple<X1, Y2> {
        PointMultiple {
            x: self.x,
            y: other.y
        }
    }
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let nums = vec![10, 45, 12, 100, -15, 0];
    println!("nums: {nums:?}");
    println!("Largest num: {}", largest(&nums));
    let chars = vec!['y', 'a', 'q', 'z', 'l'];
    println!("chars: {chars:?}");
    println!("Largest char: {}", largest(&chars));
    let point1 = Point { x: 1, y: 2};
    let point2 = Point {x: 1.0, y: 2.0};
    println!("Points: {point1:?} {point2:?}");
    println!("point1.x = {}", point1.x());
    println!("point2 distance from origin: {}", point2.distance_from_origin());
    let point3 = PointMultiple { x: 1, y: 2.0 };
    println!("Point with multiple types: {point3:?}");
    let point4 = PointMultiple { x: 'x', y: 2 };
    let point5 = point4.mixup(point3);
    println!("point4.mixup(point3): {point5:?}");
}
