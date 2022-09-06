use std::collections::{HashMap};

pub fn solve(data: &Vec<String>) -> i32 {
    let mut ptset: HashMap<(i32, i32), i32> = HashMap::new();
    for action in data {
        let a = action
        .split(" -> ")
        .flat_map( |x| { x.split(",") })
        .map( |x| { x.parse::<i32>().unwrap() })
        .collect::<Vec<i32>>();
        
        if a[0] == a[2] || a[1] == a[3] {
            let dx = i32::signum(a[2] - a[0]);
            let dy = i32::signum(a[3] - a[1]);
            let mut x = a[0];
            let mut y = a[1];
            for i in 0..(1 + i32::max(
                i32::abs(a[0] - a[2]),
                i32::abs(a[1] - a[3])
            )) {
                let point = (x, y);
                ptset.insert(
                    point,
                    ptset
                        .get(&point)
                        .unwrap_or(&0) + 1
                );
                x += dx;
                y += dy;
            }
        }
    };
    let rv = 
    ptset.values()
        .filter( |&&x| x > 1 )
        .count() as i32;
    println!("Result:\n\tCount:{}\n", rv);
    rv
}


#[test]
fn test1()
{
    let v = vec![
        "0,9 -> 5,9".to_string(),
        "8,0 -> 0,8".to_string(),
        "9,4 -> 3,4".to_string(),
        "2,2 -> 2,1".to_string(),
        "7,0 -> 7,4".to_string(),
        "6,4 -> 2,0".to_string(),
        "0,9 -> 2,9".to_string(),
        "3,4 -> 1,4".to_string(),
        "0,0 -> 8,8".to_string(),
        "5,5 -> 8,2".to_string(),
    ];
    assert_eq!(solve(&v), 5);
}
