// TODO: Implement argument handler
// TODO: check if bootdat.stat is there
// TODO: main function
// TODO: check if output is empty?

// ! bootstat -s /mnt/image -d /data/results

use core::panic;
use std::path::PathBuf;

// build the path from the source image
// and checks things like exists and valid windows dir
fn build_path(source: &str) -> PathBuf {
    let bootstat_path = "/Windows/boostat.dat";
    let mut result_string = String::from(source);
    result_string.push_str(bootstat_path);
    let mut result = PathBuf::from("/");

    // check if source is a vaild windows source dir
    let tmp = PathBuf::from(source);
    let mut tmp_windows = PathBuf::from(source);
    tmp_windows.push("Windows");

    if !(tmp.exists()) {
        panic!(
            "Source directory dosen't exists {}.",
            tmp.display().to_string()
        );
    } else if !(tmp_windows.exists()) {
        panic!(
            "Source directory is not a vaild windows root directory. Path {} not found.",
            tmp_windows.display().to_string()
        );
    }

    for s in result_string.split("/") {
        if s != "" {
            result.push(s);
        }
    }
    // check if file exists
    if result.exists() {
        return result;
    } else {
        panic!(
            "The Bootstat.dat file dosen't exists! {}",
            result.display().to_string()
        );
    }
}

fn main() {
    build_path("/Users/hakkabara/test/mnt/images/DC");
}
