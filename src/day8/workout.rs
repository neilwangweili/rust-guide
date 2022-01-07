use crate::day8::cache::Cache;
use std::thread;
use std::time::Duration;

pub fn workout(intensity: i32, random_number: i32) -> String {
    let mut cache = Cache::new(|intensity| {
        println!("Calculating now...");
        thread::sleep(Duration::from_secs(1));
        intensity
    });
    if intensity < 25 {
        format!("Today, do {} push ups!", cache.get(intensity))
            + &format!(" Next, do {} sit ups!", cache.get(intensity))
    } else if random_number == 3 {
        String::from("Take a break today.")
    } else {
        format!("Today, run {} circles.", cache.get(intensity))
    }
}
