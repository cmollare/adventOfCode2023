use regex::Regex;

#[derive(Debug)]
struct Data {
    data: Vec<Vec<i128>>,
    new_value: i128
}

pub fn day9_part2(t: &str) {
    let res : i128 = t.lines().map(parse_line).map(get_data).map(get_new_data).map(|d| d.new_value).sum();

    println!("res {:?}", res);
}

fn get_new_data(data: Data) -> Data {
    let mut to_sub_with = 0;

    for i in (0..data.data.len()).rev() {
        let last_value = data.data[i].first().unwrap();

        to_sub_with = last_value - to_sub_with;

        println!("vec {} <- {:?}", to_sub_with, data.data[i])
    }

    println!("-----------------------------------------------");

    return Data {
        data: data.data.clone(),
        new_value: to_sub_with
    };
}

fn get_data(origin: Vec<i128>) -> Data {
    let mut res = Vec::<Vec<i128>>::new();
    res.push(origin.clone());

    let mut prev = origin;
    //println!("vec {:?}", prev);
    loop {
        let tmp: Vec<_> = (0..(prev.len()-1)).map(|i| prev[i+1]-prev[i]).collect();
        res.push(tmp.clone());
        prev = tmp;

        //println!("vec {:?}", prev);

        if prev.iter().all(|v| *v == 0) { break }
    }

    //println!("---------------------------------------------------------------", );

    

    return Data {
        data: res,
        new_value: 0
    }
}

fn parse_line(l: &str) -> Vec<i128> {
    let re = Regex::new(r"(-?\d+)").unwrap();
    return re.find_iter(l).map(|c| c.as_str()).map(|s| s.parse::<i128>().unwrap()).collect();
}