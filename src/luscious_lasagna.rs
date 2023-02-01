fn main() {
    expected_minutes_in_oven();
    remaining_minutes_in_oven(30);
    preparation_time_in_minutes(2);
    elapsed_time_in_minutes(2, 28);
}
fn expected_minutes_in_oven() {
    println!("{} expected minutes in the oven", 40);
}
fn remaining_minutes_in_oven(min: i32) {
    println!("{} remaining minutes in the oven", 40 - min);
}
fn preparation_time_in_minutes(min: i32){
    println!("{} prep time remaining", 6 - min);
}
fn elapsed_time_in_minutes(layers: i32, time: i32 ) { //layers on the lasangna and time spent in the oven
    println!("{} minutes elapsed", 6 + (40-time));
}
