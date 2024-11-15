struct Point<T> {
    x: T,
    y: T,
}
struct Point2<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point2<T, U>{
    fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W>{
        Point2{
            x: self.x,
            y: other.y,
        }
    }
}

pub fn run(){
    let number_list = vec![34, 50, 25, 100, 65];
    let mut largest = number_list[0];
    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    println!("The largest number is {}", largest);

    let test_number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = get_largest(&test_number_list);
    println!("The largest number is {}", result);

    let test_alphabet_list = vec!['a', 'z', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm'];
    let result = get_largest(&test_alphabet_list);
    println!("The largest alphabet is {}", result);

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    let integer_result = get_point_largest(&integer);
    let float_result = get_point_largest(&float);
    println!("The largest integer is {}", integer_result);
    println!("The largest float is {}", float_result);

}

fn get_largest<T: PartialOrd + Copy>(list: &[T]) -> T{
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn get_point_largest<T: PartialOrd + Copy>(point: &Point<T>) -> T{
    let mut largest = point.x;
    if point.y > largest {
        largest = point.y;
    }
    largest
}