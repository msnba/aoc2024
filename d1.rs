use std::fs::File;
use std::io::{BufRead, BufReader};

fn main(){
    let input = "d1data.txt";
    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();

    let file = match File::open(input){
        Ok(file) => file,
        Err(err) => panic!("could not open file: {}", err), 
    };

    let reader = BufReader::new(file);

    for line in reader.lines() {
        if let Ok(line) = line { 
            let mut parts = line.split("   ");

            let first = parts.next();
            let second = parts.next();

            if let (Some(p1), Some(p2)) = (first, second){ //returns an option (value that may or may not exist)
                if let (Ok(n1), Ok(n2))  = (p1.trim().parse::<i32>(), p2.trim().parse::<i32>()){ //trim and parse
                    list1.push(n1);
                    list2.push(n2);
                } 
            }
        }
    }

    list1.sort();
    list2.sort();

    //TWO WAYS TO DO NEXT PART
    //==OPTION 1==
    let mut result = 0;
    for (a,b) in list1.iter().zip(list2.iter()) {
        result += (a-b).abs();
    }

    //==OPTION 2== (i think its same time complexity)
    let result2: i32 = list1
        .iter()
        .zip(list2.iter())
        .map(|(a,b)| (a-b).abs())
        .sum();

    // println!("List1: {:?}", list1);
    // println!("List2: {:?}", list2);
    println!("Result: {}", result);
    println!("Result 2: {}", result2);
}