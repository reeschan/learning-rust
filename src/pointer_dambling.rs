pub fn run(){
    let mut s1 = String::from("Hello");
    let s2 = &mut s1;
    println!("{}", s2);
    println!("{}", s1);

    
}