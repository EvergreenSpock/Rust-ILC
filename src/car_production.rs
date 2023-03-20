fn production_rate_per_hour(cars: f32) {
    if cars <= 4.0 {
        println!("{}", cars * 221.0);
    }
    if cars <= 8.0 {
        println!("{}", cars * (221.0 * 0.9));
    }
    if cars <= 10.0 {
        println!("{}", cars * (221.0 * 0.77));
    }
}
fn main() {
    production_rate_per_hour(11.0);
}