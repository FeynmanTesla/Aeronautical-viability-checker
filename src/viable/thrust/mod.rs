use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn find_thrust_at_altitude_from_map(altitude_metres: i64, altitude_thrust_map: &Vec<(i64, f64)>) -> f64 {
    let mut first_larger_or_equal_altitude_index: i64 = 0;
    for (current_altitude, _thrust) in altitude_thrust_map {
        if *current_altitude < altitude_metres { break; } else {
            first_larger_or_equal_altitude_index += 1;
        }
    }

    if altitude_thrust_map[first_larger_or_equal_altitude_index as usize].0 == altitude_metres {
        return altitude_thrust_map[first_larger_or_equal_altitude_index as usize].1;
    } else if first_larger_or_equal_altitude_index >= 2 { // linearly extrapolate val from values before
        let x_one: i64 = altitude_thrust_map[first_larger_or_equal_altitude_index as usize - 2].0;
        let x_two: i64 = altitude_thrust_map[first_larger_or_equal_altitude_index as usize - 1].0;
        let y_one: f64 = altitude_thrust_map[first_larger_or_equal_altitude_index as usize - 2].1;
        let y_two: f64 = altitude_thrust_map[first_larger_or_equal_altitude_index as usize - 1].1;

        let res: f64 = (y_one + ((((altitude_metres - x_one) / (x_two - x_one)) as f64) * (y_two - y_one))) as f64;
        return res;
    } else { // linearly extrapolate val from values after
        let x_one: i64 = altitude_thrust_map[first_larger_or_equal_altitude_index as usize + 2].0;
        let x_two: i64 = altitude_thrust_map[first_larger_or_equal_altitude_index as usize + 1].0;
        let y_one: f64 = altitude_thrust_map[first_larger_or_equal_altitude_index as usize + 2].1;
        let y_two: f64 = altitude_thrust_map[first_larger_or_equal_altitude_index as usize + 1].1;

        let res: f64 = (y_one + ((((altitude_metres - x_one) / (x_two - x_one)) as f64) * (y_two - y_one))) as f64;
        return res;
    }
}

pub fn get_altitudes_thrusts(csv_file_path: &str) -> Vec<(i64, f64)> {
    assert!(csv_file_path.ends_with(".csv"));
    let mut res: Vec<(i64, f64)> = Vec::new();

    let file = File::open(csv_file_path).expect("no such file");
    let buf = BufReader::new(file);
    let lines: Vec<String> = buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();

    assert_eq!(lines[0], "Altitude (metres), Maximum thrust (newtons)");

    for line_index in 1..(lines.len() - 1) {
        let line: String = String::from(&lines[line_index]);
        let split: Vec<&str> = line.split(",").collect();
        let altitude: i64 = split[0].trim().parse::<i64>().unwrap();
        let thrust: f64 = split[0].trim().parse::<f64>().unwrap();
        res.push((altitude, thrust));
    }

    res
}