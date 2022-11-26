// TODO: Implement argument handler
// TODO: check if output is empty?

// ! bootstat -s /mnt/image -d /data/results

use core::panic;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::Read;
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

// extracting boot times from the bootstat.dat

fn extract_boot_times(path_to_bootstat: &PathBuf) -> io::Result<()> {
    let header_size = 0x800 as u64;
    let f = &File::open(path_to_bootstat.display().to_string())?;
    let mut reader = BufReader::new(f);
    let mut buffer = Vec::new();

    let f_size = f.metadata().unwrap().len();

    // check if valid .dat size 0x1000 + header (0x800) = 67584 bytes
    // ÷https://www.geoffchappell.com/studies/windows/ie/wininet/api/urlcache/indexdat.htm
    if f_size != (0x10000 + header_size) as u64 {
        panic!("unsupported file size {} Bytes bootstat.dat.", f_size)
    }

    // Read file into vector.
    reader.read_to_end(&mut buffer)?;
    let mut current_pos = header_size;

    // check version

    // Read.
    for value in buffer {
        // println!("BYTE: {}", value);
    }
    Ok(())
}
fn main() {
    // build_path("/Users/hakkabara/test/mnt/images/DC");
    extract_boot_times(&PathBuf::from(
        "/Users/hakkabara/code/extract-bootstat/example/bootstat.dat",
    ));
}
