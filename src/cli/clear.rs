use crate::{domain::Data, utils::io_utils};

pub fn exec(data: &mut Data) {
    if io_utils::confirm("delete all data") {
        *data = Data::empty();
        println!("All data has been deleted");
    } else {
        println!("Data was not deleted");
    }
}
