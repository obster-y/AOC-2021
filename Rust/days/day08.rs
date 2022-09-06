pub fn solve(data: &Vec<String>) -> usize {
    let rv = data
    .iter()
    .map(|x| {
        x
        .split(" | ")
        .map(|y| {
            y
            .split_whitespace()
            .filter(|s| match s.len() {
                2 | 3 | 4 | 7 => true,
                _ => false
            })
        })
        .last()
        .unwrap()
        .count()
    }).sum();

    println!("Result:\n\tCount:{}", rv);
    rv
}


#[test]
fn test1()
{
    let v = vec![
        "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf".to_string()
    ];
    assert_eq!(solve(&v), 0);
}
#[test]
fn test2()
{
    let v = vec![
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe".to_string(),
        "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc".to_string(),
        "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg".to_string(),
        "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb".to_string(),
        "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea".to_string(),
        "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb".to_string(),
        "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe".to_string(),
        "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef".to_string(),
        "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb".to_string(),
        "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce".to_string(),
    ];
    assert_eq!(solve(&v), 26);
}
