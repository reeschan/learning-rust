pub fn run(){
    let s1 = String::from("Hello");
    let s2 = s1;
    println!("The value of s2 is: {}", s2);

    let i1 = 1;
    let i2 = i1;
    println!("The value of i2 is: {} {}", i1, i2);
    println!("Address of l1 and l2 is: {:p} {:p}", &i1, &i2);

    let sl1 = "literal";
    let sl2 = sl1;
    println!("Address of sl1 and sl2 is: {:p} {:p}", &sl1, &sl2);

    let s3 = s2.clone();
    println!("The value of s2 and s3 is: {} {}", s2, s3);

    let s5 = String::from("hello");
    take_ownership(s5);
    // println!("The value of s5 is: {}", s5);

    let mut s6 = String::from("hello");
    s6 = take_giveback_ownership(s6);

    let s7 = String::from("hello");
    let len = calculate_length(&s7);
    println!("The length of s7 is: s7:{} len:{}", s7, len);

    let s8 = String::from("hello");
    let s9 = push(&s8,  "world!");

    println!("The value of s8 is: {} s9:{}", s8, s9);

    let mut s10 = String::from("hello");
    let s11 = &s10;
    println!("The value of s11 is: {}", s11);

}

fn take_ownership(s: String){
    println!("The value of s is: {}", s);
}

fn take_giveback_ownership(s: String) -> String{
    s
}

fn calculate_length(s: &String) -> usize{
    s.len()
}

fn push(s: &String, push: &str) -> String{
    let mut s = s.clone();
    s.push_str(push);
    s
}