use std::io;
fn main() {

    let mut temperature: String = String::new();
    println!("CONVERT TEMPERATURES FROM CELSIUS TO FARENHEIT");
    println!("Please enter your temperature in C° : ");
    
    io::stdin()
    .read_line(&mut temperature)
    .expect("Failed to read line");


    let temperature: f64 = temperature
    .trim()
    .parse()
    .expect("Please enter a valid temperatura");

    celsius_to_farenheit(temperature);

}

fn celsius_to_farenheit(x: f64){ 
    let celsius = x;
    let farenheit = celsius * (9.0/5.0);
    let farenheit = farenheit + 32.0;
    println!("The temperature in Celsius {celsius} C° is converted to {farenheit:.2} °F");
}