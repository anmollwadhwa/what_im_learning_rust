struct Point {
    x: i64,
    y : i64,
    z : i64,
}

fn new_point(x : i64, y: i64, z: i64 ) -> Point {
    Point {x: x, y: y, z: z}
}

fn main(){
    let point = Point{ x: 1, y: 2, z: 3 };

    let x = point.x;

    let Point { x, y, z } = point;

    let Point { x, y: _, z } = point;

    let Point { x, z, .. } = point; //means that all other fields pretend underscore

    let mut point = Point { x : 0, y: 1, z: 2 };
    point.x = 5;

}