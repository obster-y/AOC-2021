pub fn solve(data: &Vec<String>) -> i32 {
    let mut width: i32 = 0;
    let mut depth: i32 = 0;

    for row in data {
        let mut val: i32 = 0;
        for v in row.split_whitespace().rev() {
            match v.parse::<i32>() {
                Ok(_) => {val = v.parse::<i32>().unwrap()},
                Err(_) => {
                    match v as &str {
                        "up" => {depth -= val},
                        "down" => {depth += val},
                        "forward" => {width += val},
                        _ => {},
                    }
                },
            }
        }
    }

    println!("Result:\nwidth:{}, depth:{}\n", width, depth);
    width * depth
}


#[test]
fn test1()
{
    let v = vec![
        "forward 5".to_string(),
        "down 5".to_string(),
        "forward 8".to_string(),
        "up 3".to_string(),
        "down 8".to_string(),
        "forward 2".to_string(),
    ];
    assert_eq!(solve(&v), 150);
}
