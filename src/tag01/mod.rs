pub fn teil1() {
    let mut number = 0;
    let mut var_alt = -1;
    let input_text = std::fs::read_to_string("src/tag01/Input.txt").unwrap();
    let mut data = Vec::new();
    for line in input_text.split('\n') {
        let s = line.replace('\r',"");
        let i = s.parse::<i32>().unwrap();
        data.push(i);
    }

    for line in data.iter(){
        if (line > &var_alt) & (var_alt != -1) {
            number = number + 1;
        }
        var_alt = *line;
    }
    println!("Anzahl: {}", number);
}

pub fn teil2() {
    let mut number = 0;
    let mut var_alt = -1;
    let mut increased = 0;
    let input_text = std::fs::read_to_string("src/tag01/Input.txt").unwrap();
    let mut data = Vec::new();
    for line in input_text.split('\n') {
        let s = line.replace('\r',"");
        let i = s.parse::<i32>().unwrap();
        data.push(i);
    }
    for (pos, element) in data.iter().enumerate(){
        if pos > 2 {
            let sum = data[pos] + data[pos - 1] + data[pos - 2];
            let sum_alt = data[pos - 1] + data[pos - 2] + data[pos - 3];
            if sum > sum_alt {
                println!("Increased");
                increased = increased + 1;
            }
            if sum < sum_alt {
                println!("Decreased");
            }
            if sum == sum_alt {
                println!("Same");
            }
        }
    }
    println!("Increased: {}", increased);
}