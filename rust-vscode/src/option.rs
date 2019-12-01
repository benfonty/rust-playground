pub fn options() {
    let toto = None;
    let titi = Some(5);
    println!("{}", toto.unwrap_or(3));
    println!("{}", titi.unwrap_or(3));
}