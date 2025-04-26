struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let red = Color(100, 0, 0);
    set_bg_color(red);

    let point = Point(30, 40, 90);
    move_point(point);
}

fn set_bg_color(color: Color) {
    println!(
        "Setting Background Color R={}, G={}, B={}",
        color.0, color.1, color.2
    )
}

fn move_point(point: Point) {
    println!(
        "The cursor was moved X={} Y={} Z={}",
        point.0, point.1, point.2
    )
}
