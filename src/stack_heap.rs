pub fn run() {
    // let a1: [u8; 9000000] = [1; 9000000];

    let mut v1 = vec![1, 2, 3, 4, 5];
    println!("The value of v1 is: {:p}", &v1);
    let v2 = vec![1, 2, 3, 4, 5];
    let mut v3 = vec![1, 2, 3, 4, 5];
    println!("Stack address of v1 is: {:p}", &v1);
    println!("Heap address of v2 is: {:p}", &v2);

    println!("The value of v1 is: {:p}", v1.as_ptr());
    println!("Length of v1 is: {}", v1.len());

    v1.insert(2, 20);
    println!("The value of v1 is: {:?}", v1);

    v1.append(&mut v3);
    println!("The value of v1 is: {:?}", v1);
}