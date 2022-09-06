mod utils;
use utils::timer::Timer;
use utils::dataio;

mod days;

fn main() {
    {
        // Day 01
        let _timer = Timer::new();
        let data_d1: Vec<i32> = dataio::read_data("data/day01.txt");
        days::day01::solve(&data_d1);
    }
}