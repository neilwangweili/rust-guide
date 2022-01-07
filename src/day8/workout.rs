use std::thread;
use std::time::Duration;

pub fn workout(intensity: i32, random_number: i32) -> String {
    let mut report = String::new();
    if intensity < 25 {
        report = format!("Today, do {} push ups!", generate_workout(intensity));
        report += &format!(" Next, do {} sit ups!", generate_workout(intensity));
    } else {
        if random_number == 3 {
            report = String::from("Take a break today.");
        } else {
            report = format!("Today, run {} circles.", generate_workout(intensity));
        }
    }
    report
}

fn generate_workout(intensity: i32) -> i32 {
    println!("Calculating now...");
    thread::sleep(Duration::from_secs(1));
    intensity
}
