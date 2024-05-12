// #![allow(unused)]

use sysinfo::{Disks, System};

fn main() {
    let mut systeminfo = System::new_all();
    systeminfo.refresh_all();

    let disks = Disks::new_with_refreshed_list();
    for disk in disks.list() {
        println!(
            "Drive: {:?}
            Used Space {:?}
            Total Space {:?}",
            disk.name(),
            disk.available_space(),
            disk.total_space()
        );
    }
}
