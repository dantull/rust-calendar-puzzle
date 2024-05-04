mod geometry;

mod stringify;
use stringify::convert_to_points;
use stringify::convert_to_strings;

fn main() {
    let shape = vec![
        "****",
        "*  *",
        "*  *",
        "****",
    ];

    let points = convert_to_points(&shape, " ");

    // Print the points
    for point in &points {
        println!("({}, {})", point.x, point.y);
    }

    let strs = convert_to_strings(&points, |_| 'x');

    let result = strs.join("\n");
    println!("{}", result);
}
