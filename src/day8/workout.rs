use std::thread;
use std::time::Duration;

pub fn workout(intensity: i32, random_number: i32) -> String {
    let expensive_closure = |intensity: i32| {
        println!("Calculating now...");
        thread::sleep(Duration::from_secs(1));
        intensity
    };
    if intensity < 25 {
        format!("Today, do {} push ups!", expensive_closure(intensity))
            + &format!(" Next, do {} sit ups!", expensive_closure(intensity))
    } else if random_number == 3 {
        String::from("Take a break today.")
    } else {
        format!("Today, run {} circles.", expensive_closure(intensity))
    }
}
