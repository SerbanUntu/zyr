use crate::{domain::Data, utils::io_utils};

/// Implementation of the `zyr clear` command
pub fn exec(data: &mut Data) {
    if io_utils::confirm("delete all data") {
        *data = Data::empty();
        println!("All data has been deleted");
    } else {
        println!("Data was not deleted");
    }
}
