pub fn solve(data: &[i32]) -> i32 {
    let mut rv = 0;

    for i in 0..data.len()-1 {
        rv += if data[i] < data[i+1] { 1 } else { 0 };
    }

    println!("Result:\n{}\n", rv);
    rv
}


#[test]
fn test1()
{
    let v = vec![
        199,
        200,
        208,
        210,
        200,
        207,
        240,
        269,
        260,
        263];
    assert_eq!(solve(&v), 7);
}

#[test]
fn test2()
{
    let v = vec![
        607,
        618,
        618,
        617,
        647,
        716,
        769,
        792];
    assert_eq!(solve(&v), 5);
}
