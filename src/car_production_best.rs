<<<<<<< HEAD
pub fn production_rate_per_hour(speed: u8) -> f64 {
    if speed <= 4 {
       return speed as f64 * 221.0
        
    } else if speed >=5 && speed <= 8 
    {
        return 0.9 *((speed as f64) * 221.0)
    }
    else if speed >=9 && speed <= 10 
    {
         return 0.77 *((speed as f64) * 221.0) //I needed to cast 'speed as f64'
    }
    else {
        println!("oops! That's wrong!");
    }
    unimplemented!("calculate hourly production rate at speed: {speed}")
}



pub fn working_items_per_minute(speed: u8) -> u32 {
    let cpm = 221.0 / 60.0; //cars per minute
    if speed <= 4 {
        return (speed as f64 * cpm) as u32
         
     } else if speed >=5 && speed <= 8 
     {
         return (0.9 *((speed as f64) * cpm)) as u32
     }
     else if speed >=9 && speed <= 10 
     {
          return (0.77 *((speed as f64) * cpm)) as u32 //I needed to cast 'speed as f64'
     }
     else {
         println!("oops! That's wrong!");
     }
    unimplemented!("calculate the amount of working items at speed: {speed}")
}




fn main() {
println!("{}", production_rate_per_hour(5));
println!("{}", working_items_per_minute(6));
=======
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
>>>>>>> 3aac5e584e5e5fb0fc3fd94bea9b7a06810d3ec2
}