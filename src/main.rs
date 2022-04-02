use prez::{Line, Point};

fn main() {
    let p = Point { x: 0.0, y: 0.0 };

    let l = Line::new(1.0, 2.0, 0.0);

    println!("is point {} on line {}? {}", &p, &l, l.intersects_point(&p)); // true

    let l1 = Line::new(1.0, 0.0, 3.0);
    let l2 = Line::new(-5.0, 0.0, -3.0);

    println!(
        "do lines ({}, {}) intersect? {}",
        &l1,
        &l2,
        l1.is_parallel(&l2)
    ); // true
}
