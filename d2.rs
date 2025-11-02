use std::fs::File;
use std::io::{BufRead,BufReader};

fn main(){
    let input = "d2data.txt";

    let mut safe: i32 = 0;

    let file = match File::open(input){
        Ok(file) => file,
        Err(err) => panic!("could not open file: {}",err)
    };

    let reader = BufReader::new(file);

    for line in reader.lines(){
        if let Ok(line) = line {
            let levels: Vec<i32> =  line.split(" ").filter_map(|s| s.parse().ok()).collect();

            let increase = levels[1] > levels[0];
            let mut is_safe = true;

            for i in 0 .. levels.len()-1 { //.enumerate gets value and idx
                let difference = levels[i+1] - levels[i];
                if (difference > 0) != increase || difference.abs() < 1 || difference.abs() > 3 {
                    is_safe = false;
                    break;
                }
            }

            if is_safe {
                safe += 1;
            }
        }
    }

    println!("Safe Lines: {}", safe);
}