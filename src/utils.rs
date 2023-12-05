use std::{env::current_dir, fs::File, io::BufReader};

pub fn get_reader_for_day(day: u8) -> BufReader<File> {
    let current_dir = current_dir().expect("Can't get current directory?!");
    let in_f_path = current_dir.join("input").join(format!("day-{}.txt", day));
    let file = File::open(in_f_path.to_str().unwrap())
        .expect(format!("Really, the path ({:?}) is wrong?", in_f_path).as_str());

    BufReader::new(file)
}
