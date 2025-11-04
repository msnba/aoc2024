use std::fs; //better file reading

fn main(){
    let input = fs::read_to_string("d3data.txt").expect("could not open file");

    let mut sum: i32 = 0;
    let mut pos = 0;

    while let Some(start) = input[pos..].find("mul("){ //some basically means a value that could be missing or invalid, every iteration start from pos and find mul(
        let start_idx = pos + start + 4; //start cursor from after (

        if let Some(end_idx) = input[start_idx..].find(')'){
            let end_idx = start_idx + end_idx;
            let inside = &input[start_idx..end_idx]; //x,x

            let parts: Vec<&str> = inside.split(',').collect(); //.collect gathers .split (an iterator) into a collection (Vec in this case)

            if parts.len() == 2{
                if let (Ok(a), Ok(b)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()){//parse strings to i32
                    println!("mul({}, {}) = {}", a, b, a * b);
                    sum += a*b;
                }
            }
            pos = pos+start+1; //ran into an error before where pos = pos+ end_position but there could be overlapping muls, so i did this to ensure finding all of the muls
        }
    }


    println!("Total sum: {}", sum);
}