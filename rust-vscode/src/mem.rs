struct Point {
    x: i32,
    y: i32,
}

struct Line {
    from: Point,
    to: Point,
}

pub fn mem() {
    let p1 = Point { x: 0, y: 0 };
    let p2 = Point { x: 1, y: 1 };

    println!(
        "{}, {}",
        std::mem::size_of_val(&p1),
        std::mem::size_of_val(&p2)
    );

    let l = Line { from: p1, to: p2 };

    println!("{}", std::mem::size_of_val(&l));
}
