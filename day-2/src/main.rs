use std::{fs::OpenOptions, io::Read};

//intcode computer
fn multiply(intcode: &mut Vec<u32>, intcode_index: usize ) {
    let index_num_1 = intcode[intcode_index+1] as usize;
    let index_num_2 = intcode[intcode_index+2] as usize;
    let index_position = intcode[intcode_index+3] as usize;
    intcode[index_position] = intcode[index_num_1] * intcode[index_num_2];
}


fn add(intcode: &mut Vec<u32>, intcode_index: usize ) {
    let index_num_1 = intcode[intcode_index+1] as usize;
    let index_num_2 = intcode[intcode_index+2] as usize;
    let index_position = intcode[intcode_index+3] as usize;
    intcode[index_position] = intcode[index_num_1] + intcode[index_num_2];
}


fn main() {

    let mut intcode_file = OpenOptions::new().read(true).open("input.txt").unwrap();
    let mut intcode_input = String::new();
    intcode_file.read_to_string(&mut intcode_input).unwrap();

    let mut intcode: Vec<u32> = vec![];

    let mut buf: String = "".to_string();
    for char in intcode_input.chars(){
        if char == ',' {
            intcode.push(buf.parse().unwrap());
            buf.truncate(0);
        } else {
            buf.push(char);
        }
    }
    intcode.push(buf.trim().parse().unwrap());
    drop(buf);
    drop(intcode_input);

    for i in (0..intcode.len()).step_by(4) {
        match intcode[i] as u8 {
            1 => add(&mut intcode, i),
            2 => multiply(&mut intcode, i),
            99 => break,
            _ => println!("Not a valid opcode! {}", intcode[i])
        }
    }
    println!("{:?}", intcode)
} 

//part 2 - solver

// fn multiply(intcode: &mut Vec<u32>, intcode_index: usize ) {
//     let index_num_1 = intcode[intcode_index+1] as usize;
//     let index_num_2 = intcode[intcode_index+2] as usize;
//     let index_position = intcode[intcode_index+3] as usize;
//     intcode[index_position] = intcode[index_num_1] * intcode[index_num_2];
// }


// fn add(intcode: &mut Vec<u32>, intcode_index: usize ) {
//     let index_num_1 = intcode[intcode_index+1] as usize;
//     let index_num_2 = intcode[intcode_index+2] as usize;
//     let index_position = intcode[intcode_index+3] as usize;
//     intcode[index_position] = intcode[index_num_1] + intcode[index_num_2];
// }


// fn main() {

//     let mut intcode_file = OpenOptions::new().read(true).open("input.txt").unwrap();
//     let mut intcode_input = String::new();
//     intcode_file.read_to_string(&mut intcode_input).unwrap();

//     let mut intcode: Vec<u32> = vec![];

//     let mut buf: String = "".to_string();
//     for char in intcode_input.chars(){
//         if char == ',' {
//             intcode.push(buf.parse().unwrap());
//             buf.truncate(0);
//         } else {
//             buf.push(char);
//         }
//     }
//     intcode.push(buf.trim().parse().unwrap());
//     drop(buf);
//     drop(intcode_input);
//     for  j in 1..=100 {
//         for k in 1..=100 {
//             let mut intcode_copy = intcode.clone();
//             intcode_copy[1] = j;
//             intcode_copy[2] = k;
//             for i in (0..intcode_copy.len()).step_by(4) {
//                 match intcode_copy[i] as u8 {
//                 1 => add(&mut intcode_copy, i),
//                 2 => multiply(&mut intcode_copy, i),
//                 99 => break,
//                 _ => println!("Not a valid opcode! {}", intcode_copy[i])
//                 }
//             }
//         if intcode_copy[0] == 19690720 {
//             println!("noun:{}, verb:{}", j,k);
//         }
//         }

//     }
//     println!("{:?}", intcode)
// } 

