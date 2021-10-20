pub fn degree_convert(input: f64, unit: &str, target_unit: &str) -> String {
    const FAHRENHEIT: f64 = 1.0;
    const CELSIUS: f64 = 1.0 / 1.8;
    const CELSIUS_ERROR_VALUE: f64 = -32.0 / 1.8;
    let mut value: f64 = 0.0;
    if unit.eq("FAHRENHEIT") {
        value = input / FAHRENHEIT;
    } else {
        value = (input - CELSIUS_ERROR_VALUE) / CELSIUS;
    }
    return format!("{:.2}", if target_unit.eq("FAHRENHEIT") {
        value * FAHRENHEIT
    } else {
        value * CELSIUS + CELSIUS_ERROR_VALUE
    });
}