// use std::collections::{HashMap};

pub fn solve(data: &Vec<String>) -> i64 {
    const NEW_DAY: i64 = 9;
    const ORIGIN_DAY: i64 = 7;
    let mut school: Vec<(i64, i64)> = data[0]
        .split(",")
        .map( |x| (x.parse::<i64>().unwrap(), 0i64) )
        .collect();
    for day in 0..data[1].parse::<i64>().unwrap() {
        // let n = school
        //     .iter()
        //     .filter_map( |(x, add_day)| {
        //         let dif = (day - (x + add_day));
        //         if dif >= 0 && dif % ORIGIN_DAY == 0 {
        //             return Some(1);
        //         }
        //         else {
        //             return None;
        //         }
        //     })
        //     .count() as usize;

        let n = school
            .iter()
            .fold(0, |sum, (x, add_day)| {
                let dif = day - (x + add_day);
                if dif >= 0 && dif % ORIGIN_DAY == 0 {
                    return sum + 1;
                }
                else {
                    return sum;
                }
            }) as usize;

        school.append(&mut vec![(NEW_DAY-1, day+1); n]);
    }
    println!("Result:\n\tCount:{}", school.len());
    school.len() as i64
}


#[test]
fn test1()
{
    let v = vec![
        "3,4,3,1,2".to_string(),
        "18".to_string(),
    ];
    assert_eq!(solve(&v), 26);
}
#[test]
fn test2()
{
    let v = vec![
        "3,4,3,1,2".to_string(),
        "80".to_string(),
    ];
    assert_eq!(solve(&v), 5934);
}
#[test]
fn test3()
{
    let v = vec![
        "3,4,3,1,2".to_string(),
        "256".to_string(),
    ];
    assert_eq!(solve(&v), 26984457539);
}
