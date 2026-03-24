// use std::io;

// fn main() {
//     println!("Enter farenheit value:");

//     let mut farenheit = String::new();
//     io::stdin().read_line(&mut farenheit).expect("Failed to read line");

//     println!("Farenheit temp: {farenheit}");

//     let farenheit: f64 = farenheit.trim().parse().expect("Error parsing farenheit value: {farenheit}");
//     let celcius = (farenheit - 32.0) * 5.0/9.0;

//     println!("Celcius temp: {celcius}");
// }



use std::io;

fn main() {
  println!("Enter fahrenheit value:");
  loop {
    let mut temp = String::new();
    io::stdin().read_line(&mut temp).expect("test");
    println!("Value entered: {temp}");
    let temp: f64 = match temp.trim().parse() {
      Ok(num) => num,
      Err(error) => {
        println!("Error: {error}");
        continue;
      }
    };

    let celcius = (temp - 32.0) * 5.0/9.0;
    println!("Celcius temp: {celcius}");
    break
  };
}