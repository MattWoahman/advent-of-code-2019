// part one

// use std::{fs::OpenOptions, io::Read};

// extern crate geo;

// use geo::{Coord, Line, LineIntersection, coord, line_intersection::{ line_intersection}};

// fn wire_to_coordinates(wire: String) -> Vec<Coord>{
//     let mut coordinate_list= Vec::new();
//     let mut buf = "".to_string();
//     for char in wire.chars(){
//         if char == ',' {
//             coordinate_list.push(buf.clone());
//             buf.truncate(0);
//         } else {
//             buf.push(char);
//         }
//     }
//     coordinate_list.push(buf.clone());
    
//     let mut wire_coordinates = Vec::new();
//     wire_coordinates.push(Coord::zero());
//     for (i,coordinates) in coordinate_list.clone().iter().enumerate() {
//         let coordinate: Vec<char> = coordinates.chars().collect();
//         let starting_coordinate = wire_coordinates[i].clone();
//         let direction = coordinate[0];
//         let amount = &coordinate[1..coordinates.len()];
//         wire_coordinates.push(coordinate_mapper(starting_coordinate,direction, amount));
//     }
//     wire_coordinates
// }

// fn coordinate_mapper(starting_coordinate: Coord, direction: char, move_amount_chars: &[char]) -> Coord {
//     let move_amount: f64 = move_amount_chars.iter().collect::<String>().trim().parse::<f64>().unwrap();
//     let x= starting_coordinate.x;
//     let y = starting_coordinate.y;
//     match direction {
//         'U' => coord!(x:x, y:y + move_amount),
//         'D' => coord!(x:x, y:y - move_amount),
//         'L' => coord!(x:x - move_amount, y:y),
//         'R' => coord!(x:x + move_amount, y:y),
//         _ => Coord::zero()
//     }
// }

// fn coordinates_to_lines(coordinates: Vec<Coord>) -> Vec<Line> {
//     let mut lines = vec![];
//     for (i,coordinate) in coordinates.iter().enumerate() {
//         if i > 0 {
//             lines.push(Line::new(coordinates[i-1], *coordinate))
//         }
//     }
//     lines
// }

// fn main() {
//     let mut wire_file = OpenOptions::new().read(true).open("input.txt").unwrap();
//     let mut wire_input = String::new();

//     wire_file.read_to_string(&mut wire_input).unwrap();

//     let mut wires: Vec<String> = vec![];

//     for wire in wire_input.lines() {
//         wires.push(wire.to_string());
//     }

//     let first_wire = wires[0].clone();
//     let second_wire = wires[1].clone();
//     drop(wires);

//     let first_wire_coords = wire_to_coordinates(first_wire);
//     let second_wire_coords = wire_to_coordinates(second_wire);

//     let first_wire_lines = coordinates_to_lines(first_wire_coords);
//     let second_wire_lines = coordinates_to_lines(second_wire_coords);

//     let mut intersection_distance = vec![];

//     for first_line in &first_wire_lines{
//         for second_line in &second_wire_lines {
//             if let Some(LineIntersection::SinglePoint { intersection, is_proper }) = line_intersection(*first_line, *second_line) {
//                 if is_proper {
//                     let distance = (intersection.x.abs() + 0.0) + (intersection.y.abs() + 0.0);
//                     intersection_distance.push(distance);
//                 }
//             }
//         }
//     }
    
//     let mut min_distance = 0.0;
//     for intersection in intersection_distance {
//         if min_distance == 0.0{
//             min_distance = intersection;
//         } else if min_distance > intersection {
//             min_distance = intersection;
//         }
//     }

//     println!("{:?}", min_distance)

// }


//part two 

use std::{fs::OpenOptions, io::Read};

extern crate geo;

use geo::{Coord, Euclidean, Length, Line, LineIntersection, coord, line_intersection::line_intersection};

fn wire_to_coordinates(wire: String) -> Vec<Coord>{
    let mut coordinate_list= Vec::new();
    let mut buf = "".to_string();
    for char in wire.chars(){
        if char == ',' {
            coordinate_list.push(buf.clone());
            buf.truncate(0);
        } else {
            buf.push(char);
        }
    }
    coordinate_list.push(buf.clone());
    
    let mut wire_coordinates = Vec::new();
    wire_coordinates.push(Coord::zero());
    for (i,coordinates) in coordinate_list.clone().iter().enumerate() {
        let coordinate: Vec<char> = coordinates.chars().collect();
        let starting_coordinate = wire_coordinates[i].clone();
        let direction = coordinate[0];
        let amount = &coordinate[1..coordinates.len()];
        wire_coordinates.push(coordinate_mapper(starting_coordinate,direction, amount));
    }
    wire_coordinates
}

fn coordinate_mapper(starting_coordinate: Coord, direction: char, move_amount_chars: &[char]) -> Coord {
    let move_amount: f64 = move_amount_chars.iter().collect::<String>().trim().parse::<f64>().unwrap();
    let x= starting_coordinate.x;
    let y = starting_coordinate.y;
    match direction {
        'U' => coord!(x:x, y:y + move_amount),
        'D' => coord!(x:x, y:y - move_amount),
        'L' => coord!(x:x - move_amount, y:y),
        'R' => coord!(x:x + move_amount, y:y),
        _ => Coord::zero()
    }
}

fn coordinates_to_lines(coordinates: Vec<Coord>) -> Vec<Line> {
    let mut lines = vec![];
    for (i,coordinate) in coordinates.iter().enumerate() {
        if i > 0 {
            lines.push(Line::new(coordinates[i-1], *coordinate))
        }
    }
    lines
}

fn main() {
    let mut wire_file = OpenOptions::new().read(true).open("input.txt").unwrap();
    let mut wire_input = String::new();

    wire_file.read_to_string(&mut wire_input).unwrap();

    let mut wires: Vec<String> = vec![];

    for wire in wire_input.lines() {
        wires.push(wire.to_string());
    }

    let first_wire = wires[0].clone();
    let second_wire = wires[1].clone();
    drop(wires);

    let first_wire_coords = wire_to_coordinates(first_wire);
    let second_wire_coords = wire_to_coordinates(second_wire);

    let first_wire_lines = coordinates_to_lines(first_wire_coords);
    let second_wire_lines = coordinates_to_lines(second_wire_coords);

    let mut intersected_first_lines = vec![];
    let mut intersected_second_lines = vec![]; 

    let mut intersection_distance = vec![];
    let mut intersections = vec![];

    for first_line in &first_wire_lines{
        for second_line in &second_wire_lines {
            if let Some(LineIntersection::SinglePoint { intersection, is_proper }) = line_intersection(*first_line, *second_line) {
                if is_proper {
                    let distance = (intersection.x.abs() + 0.0) + (intersection.y.abs() + 0.0);
                    intersection_distance.push(distance);
                    intersected_first_lines.push(first_line);
                    intersected_second_lines.push(second_line);
                    intersections.push(intersection);
                }
            }
        }
    }

    let mut first_line_distances = vec![];
    let mut second_line_distances = vec![];
    let mut total_distances = vec![];

    for (index,first_line) in intersected_first_lines.iter().enumerate() {
        let mut first_line_distance = 0.0;
        let mut second_line_distance = 0.0;
        let second_line = intersected_second_lines[index];
        if let Some(wire_index) = first_wire_lines.iter().position(|x| x == *first_line ) {
            for i in 0..wire_index {
                first_line_distance += Euclidean.length(&first_wire_lines[i])
            }

        if let Some(wire_index) = second_wire_lines.iter().position(|x| x == second_line) {
                for i in 0..wire_index {
                    second_line_distance += Euclidean.length(&second_wire_lines[i])
            }
        }
            let first_line_start_coord = coord!{x: first_line.start_point().x(), y: first_line.start_point().y()};
            let second_line_start_coord = coord!{x: second_line.start_point().x(), y: second_line.start_point().y()};
            first_line_distance += Euclidean.length(&Line::new(first_line_start_coord, intersections[index]));
            second_line_distance += Euclidean.length(&Line::new(second_line_start_coord,intersections[index]));
        }
        first_line_distances.push(first_line_distance);
        second_line_distances.push(second_line_distance);
        total_distances.push(first_line_distance + second_line_distance);
    }
    
    println!("intersected First Line Len: {}", intersected_first_lines.len());
    println!("Intersected Second Line Len: {}", intersected_second_lines.len());
    println!("Intersections: {}", intersections.len());

    let mut min_distance = 0.0;
    for distance in total_distances {
        if min_distance == 0.0{
            min_distance = distance;
        } else if min_distance > distance {
            min_distance = distance;
        }
    }

    println!("{:?}", min_distance)

}
