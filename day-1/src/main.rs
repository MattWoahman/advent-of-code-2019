use std::{fs::OpenOptions, io::Read};

fn fuel_calculation(mass: f32) -> f32 {
    let mut new_mass = ((mass/3.0).floor())-2.0;
    if new_mass > 0.0 {
        new_mass += fuel_calculation(new_mass);
    } else {
        new_mass = 0.0;
    }
    new_mass
}

fn main() {
    let mut final_fuel:f32 = 0.0;

    let mut input_file = OpenOptions::new().read(true).open("input.txt").unwrap();
    let mut input_file_contents = String::new();
    input_file.read_to_string(&mut input_file_contents).unwrap();

    for input in input_file_contents.lines() {
        final_fuel += fuel_calculation(input.parse().unwrap());
    }

    println!("Final fuel: {}", final_fuel)
}
