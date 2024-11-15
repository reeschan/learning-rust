enum List {
    Node(i32, Box<List>),
    Nil,
}

pub fn run(){
    let t1: (i64, String) = (1, String::from("Hello"));
    println!("The value of t1 is: {:p}", &t1);
    println!("Heap address of t1 is: {:p}", &t1.1);
    println!("Length of t1 is: {}", t1.1.len());
    println!("Capacity of t1 is: {}", t1.1.capacity());

    let mut b1 = Box::new(t1);
    println!("The value of b1 is: {:p}", &b1);
    (*b1).1 += " world!";
    println!("The value of b1 is: {} {}", b1.0, b1.1);
}