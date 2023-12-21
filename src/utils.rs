use std::{
    env::current_dir,
    fmt::Debug,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
    vec::IntoIter,
};

pub fn get_reader_for_day(day: u8) -> BufReader<File> {
    _get_reader(format!("day-{}.txt", day))
}

pub fn get_test_reader_for_day(day: u8) -> BufReader<File> {
    _get_reader(format!("day-{}-test.txt", day))
}

fn _get_reader(file_name: String) -> BufReader<File> {
    let current_dir = current_dir().expect("Can't get current directory?!");
    let in_f_path = current_dir.join("input").join(file_name);
    let file = File::open(in_f_path.to_str().unwrap())
        .expect(format!("Really, the path ({:?}) is wrong?", in_f_path).as_str());

    BufReader::new(file)
}

pub fn lines(day: u8) -> IntoIter<String> {
    _get_lines(format!("day-{}.txt", day))
}

pub fn test_lines(day: u8) -> IntoIter<String> {
    _get_lines(format!("day-{}-test.txt", day))
}

fn _get_lines(file_name: String) -> IntoIter<String> {
    _get_reader(file_name)
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<String>>()
        .into_iter()
}

pub fn ints<T: FromStr>(line: String) -> Vec<T>
where
    <T as FromStr>::Err: Debug,
{
    line.split_ascii_whitespace()
        .map(|c| c.parse::<T>().unwrap())
        .collect()
}