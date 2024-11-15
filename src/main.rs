mod stack_heap;
mod box_pointer;
mod ownership;
mod pointer_dambling;
mod genrics;

const S: &str = "Hello, world!";
const MAX_POINTS: i32 = 100_000;

fn main() {
    println!("The value of x is: {:p}", &S);

    println!("Memory address of const is: {:p}", &MAX_POINTS);

    let i2: i64 = 1;
    let i3: i64 = 2;
    println!("Memory address of i2 is: {:p}", &i2);
    println!("Memory address of i3 is: {:p}", &i3);

    let y = 5;
    println!("The value of y is: {:p}", &y);
    let y = y+1;
    println!("The value of y is: {:p}", &y);

    println!("The value of y is: {}", y);

    {
        let y = 3;
        println!("The value of y is: {}", y);
    }

    let (_x, _y, _z) = (1, 2, 3);
    println!("The value of x is: {}", _x);

    let mut t2 = ((0,1), (2,3));
    let ((ref mut x1_ptr, ref mut y1_ptr), _) = t2;
    *x1_ptr = 5;
    *y1_ptr = -5;
    println!("The value of x1 is: {} and y1 is: {}", x1_ptr, y1_ptr);

    let a1 = [1,2,3,4,5];
    println!("The value of a1 is: {:?}", a1);

    let s1 = "Hello, world!";

    println!("The value of s1 is: {:p}", s1);

    let s2 = String::from("Hello, world!");

    println!("The value of s2 is: {:p}", &s2);

let mut s3 = String::from("Helああ");
    println!("The value of s3 is: {:p}", s3.as_ptr());
    println!("capacity of s3 is: {}", s3.capacity());

    s3.push_str("lo, world!");

    println!("The value of s3 is: {:p}", s3.as_ptr());
    println!("capacity of s3 is: {}", s3.capacity());
    println!("The value of s3 is: {}", s3);

    let s4 = "test";
    println!("The value of s4 is: {}", s4);

    stack_heap::run();

    box_pointer::run();

    ownership::run();

    pointer_dambling::run();

    genrics::run();
}
