pub fn solve(data: &Vec<String>) -> i32 {
    let mut count: Vec<i32> =vec![0; data[0].len() as usize];
    for row in data {
        for (i, c) in row.chars().enumerate() {
            match c {
                '0' => {count[i] += 1},
                '1' => {count[i] -= 1},
                _ => {},
            }
        }
    }

    let gamma = count
    .iter()
    .map(|&x| {
        if x > 0 { "1" } else { "0" }
    })
    .collect::<Vec<&str>>()
    .join("");
    let gamma = i32::from_str_radix(&gamma, 2).unwrap();

    let epsilon = count
    .iter()
    .map(|&x| {
        if x > 0 { "0" } else { "1" }
    })
    .collect::<Vec<&str>>()
    .join("");
    let epsilon = i32::from_str_radix(&epsilon, 2).unwrap();

    println!("Result:\ngamma:{}, epsilon:{}\n", gamma, epsilon);
    gamma * epsilon
}


#[test]
fn test1()
{
    let v = vec![
        "00100".to_string(),
        "11110".to_string(),
        "10110".to_string(),
        "10111".to_string(),
        "10101".to_string(),
        "01111".to_string(),
        "00111".to_string(),
        "11100".to_string(),
        "10000".to_string(),
        "11001".to_string(),
        "00010".to_string(),
        "01010".to_string(),
    ];
    assert_eq!(solve(&v), 198);
}
