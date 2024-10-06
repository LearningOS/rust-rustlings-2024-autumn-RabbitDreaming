// options3.rs
//
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a
// hint.



struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    match y {
        Some(ref p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => panic!("no match!"),
    }
    if let Some(point) = &y {  
        println!("Point was used: ({}, {})", point.x, point.y);  
    } 
}
