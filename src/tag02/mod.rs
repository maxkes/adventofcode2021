pub fn teil1() {
    let input_text = std::fs::read_to_string("src/tag02/Input.txt").unwrap();
    let mut depth = 0;
    let mut forw = 0;
    for line in input_text.split('\n') {
        let s = line.replace('\r',"");
        let mut l = s.split_whitespace();
        let command = l.next().unwrap();
        let data = l.next().unwrap().parse::<i32>().unwrap();
        //println!("Command: {:?}    Data: {:?}", command, data);
        if command == "forward" {
            forw = forw + data;
        }
        else if command == "up" {
            depth = depth - data;
        }
        else if command == "down" {
            depth = depth + data;
        }
    }
    println!("Depdh: {:?}    Foward: {:?}", depth, forw);
    println!("Multiplied: {:?}", depth*forw)
}

pub fn teil2() {
    let input_text = std::fs::read_to_string("src/tag02/Input.txt").unwrap();
    let mut depth = 0;
    let mut forw = 0;
    let  mut aim = 0;
    for line in input_text.split('\n') {
        let s = line.replace('\r',"");
        let mut l = s.split_whitespace();
        let command = l.next().unwrap();
        let data = l.next().unwrap().parse::<i32>().unwrap();
        //println!("Command: {:?}    Data: {:?}", command, data);
        if command == "forward" {
            forw = forw + data;
            depth = depth + aim * data;
        }
        else if command == "up" {
            aim = aim - data;
        }
        else if command == "down" {
            aim = aim + data;
        }
    }
    println!("Depdh: {:?}    Foward: {:?}", depth, forw);
    println!("Multiplied: {:?}", depth*forw)
}