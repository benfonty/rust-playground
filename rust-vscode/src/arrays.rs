use std::mem;


pub fn arrays_vectors() {
    let a = [1, 2, 3];
    let mut b = Vec::new();
    b.push(1);
    b.push(2);
    b.push(3);
    println!("size of a {}",std::mem::size_of_val(&a));
    println!("size of b {}",std::mem::size_of_val(&b));
}