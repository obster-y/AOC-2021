pub fn solve(data: &Vec<String>) -> Option<usize> {
    const WIDTH: usize = 5;
    const WIDTH_SQ: usize = WIDTH.pow(2);
    const WIDTH_M2: usize = WIDTH * 2;

    let actions: Vec<usize> = data
        .iter()
        .nth(0)?
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    //* Construct 3D Vector *//
    // let keyboard: Vec<Vec<Vec<usize>>> = data
    //     .iter()
    //     .skip(2)
    //     .filter_map( |a| {
    //         if a == "" {
    //             return None;
    //         }
    //         Some(a
    //             .split_whitespace()
    //             .map( |x| {
    //                 x.parse::<usize>().unwrap()
    //             })
    //             .collect::<Vec<usize>>())
    //     })
    //     .collect::<Vec<Vec<usize>>>()
    //     .chunks(WIDTH)
    //     .map(|x| x.to_vec())
    //     .collect();

    //* Construct 1D Vector *//
    let keyboard: Vec<usize> = data
        .iter()
        .skip(2)
        .flat_map( |a| {
            a
            .split_whitespace()
            .map( |x| x.parse::<usize>().unwrap() )
            .collect::<Vec<usize>>()
            .into_iter()
        })
        .collect();

    //* Construct count array to record selected item on each row/col __NO DIAG__ *//
    let mut count: Vec<Vec<usize>> = vec![
        vec![
            0; WIDTH_M2 as usize
        ]; keyboard.len() as usize];

    //* Sum each array for calc result
    let mut sum: Vec<usize> = keyboard
        .chunks(WIDTH_SQ)
        .map(|x| x.to_vec().iter().sum())
        .collect();

    //* Do for each action *//
    for action in actions {
        let x = keyboard
            .iter()
            .enumerate()
            .filter(|&(_i, v)| *v == action)
            .filter_map(|(i, v)| {
                let n_ary = i / WIDTH_SQ;
                let n_seq = i % WIDTH_SQ;
                let x = n_seq / WIDTH;
                let y = n_seq % WIDTH;

                count[n_ary][x] += 1;
                count[n_ary][y+WIDTH] += 1;
                sum[n_ary] -= v;

                if count[n_ary].iter().position(|&v| v==WIDTH) != None {
                    Some(sum[n_ary] * v)
                } else {
                    None
                }
            })
            .collect::<Vec<usize>>();
        if x != vec![] {
            println!("Result:\nScore:{}\n", x[0]);
            return Some(x[0])
        }
    }
    return None
}


#[test]
fn test1()
{
    let v = vec![
        "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1".to_string(),
        "".to_string(),
        "22 13 17 11  0".to_string(),
        " 8  2 23  4 24".to_string(),
        "21  9 14 16  7".to_string(),
        " 6 10  3 18  5".to_string(),
        " 1 12 20 15 19".to_string(),
        "".to_string(),
        " 3 15  0  2 22".to_string(),
        " 9 18 13 17  5".to_string(),
        "19  8  7 25 23".to_string(),
        "20 11 10 24  4".to_string(),
        "14 21 16 12  6".to_string(),
        "".to_string(),
        "14 21 17 24  4".to_string(),
        "10 16 15  9 19".to_string(),
        "18  8 23 26 20".to_string(),
        "22 11 13  6  5".to_string(),
        " 2  0 12  3  7".to_string(),
    ];
    assert_eq!(solve(&v), Some(4512));
}
