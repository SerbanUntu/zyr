use std::io;
use crate::domain::Data;

pub fn exec(data: &mut Data) {
    println!("Are you sure you want to clear all data? (y/N)");
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("Did not enter a correct string");

    if buf.chars().next().is_some_and(|ch| ch == 'y') {
        *data = Data::empty();
        println!("All data has been deleted");
    } else {
        println!("Data was not deleted");
    }
}
