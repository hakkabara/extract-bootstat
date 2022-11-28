// TODO: Implement argument handler

// ! bootstat -s /mnt/image -d /data/results

use core::panic;
use std::fs::File;
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
// TODO: check if we read this file readonly
fn read_bootstat(path_to_bootstat: &PathBuf) -> Vec<u8> {
    let header_size = 0x800 as u64;
    let f = &File::open(path_to_bootstat.display().to_string())
        .expect("Unable to open bootstat.dat file check permissions!");
    let mut reader = BufReader::new(f);
    let mut buffer = Vec::new();

    let f_size = f.metadata().unwrap().len();

    // check if valid .dat size 0x1000 + header (0x800) = 67584 bytes
    // ÷https://www.geoffchappell.com/studies/windows/ie/wininet/api/urlcache/indexdat.htm
    if f_size != (0x10000 + header_size) as u64 {
        panic!("unsupported file size {} Bytes bootstat.dat.", f_size)
    }

    // Read file into vector.
    reader
        .read_to_end(&mut buffer)
        .expect("unable to read bootstat.dat");
    return buffer;
}

fn get_information(buffer: Vec<u8>) {
    let mut current_pos = 0x800;
    let version = array_2_u32(&buffer[(current_pos as usize)..((current_pos as usize) + 4)]);
    if version != 4 {
        panic!("unsupported version: {} abort", version);
    }
    current_pos += 4;
    // Getting Information
    //TODO: cleaner code with for loop?
    let boot_log_start = array_2_u32(&buffer[(current_pos as usize)..((current_pos as usize) + 4)]);
    current_pos += 4;

    let boot_log_size = array_2_u32(&buffer[(current_pos as usize)..((current_pos as usize) + 4)]);
    current_pos += 4;

    let next_boot_log_entry =
        array_2_u32(&buffer[(current_pos as usize)..((current_pos as usize) + 4)]);
    current_pos += 4;

    let first_boot_log_entry =
        array_2_u32(&buffer[(current_pos as usize)..((current_pos as usize) + 4)]);

    // TODO: check if output is the same
    // print debug information in console
    println!("BootLogSize: {}", boot_log_size);
    println!("BootLogStart: {}", boot_log_start);
    println!("BootLogSize: {}", first_boot_log_entry);
    println!("BootLogSize: {}", next_boot_log_entry);

    let mut overlapx = true;

    // when first log entry is greater than the next => log is overwritten

    if (first_boot_log_entry > next_boot_log_entry) {
        overlapx = false;
        println!("Log is partially overwritten due to its circular nature.");
    }

    while (true) {
        let record_start = current_pos;
        let timestamp = array_2_u64(&buffer[(current_pos as usize)..((current_pos as usize) + 8)]);
        current_pos += 8;
        // TODO: tostring etc
        // let application_id = &buffer[(current_pos as usize..(current_pos as usize) + 16)];
        current_pos += 16;

        let entry_size = array_2_u32(&buffer[(current_pos as usize)..((current_pos as usize) + 4)]);
        current_pos += 4;
        let level = array_2_u32(&buffer[(current_pos as usize)..((current_pos as usize) + 3)]);
        current_pos += 4;
        let application_type =
            array_2_u32(&buffer[(current_pos as usize)..((current_pos as usize) + 3)]);
        current_pos += 4;
        let event_code = array_2_u32(&buffer[(current_pos as usize)..((current_pos as usize) + 3)]);
        current_pos += 4;

        println!("record start: {}", record_start);
        println!("timestamp: {}", timestamp);
        // println!("application ID: {}", application_id);
        println!("entry size: {}", entry_size);
        println!("level: {}", level);
        println!("Application Type: {}", application_type);
        println!("event code: {}", event_code);

        if ((application_type == 3) && (event_code == 1)) {}
    }
}

fn array_2_u32(b: &[u8]) -> u32 {
    let ulong: u32 =
        ((b[3] as u32) << 24) | ((b[2] as u32) << 16) | ((b[1] as u32) << 8) | (b[0] as u32);
    return ulong;
}

fn array_2_u64(b: &[u8]) -> u64 {
    let ulong: u64 = ((b[7] as u64) << 56)
        | ((b[6] as u64) << 48)
        | ((b[5] as u64) << 40)
        | ((b[4] as u64) << 32)
        | ((b[3] as u64) << 24)
        | ((b[2] as u64) << 16)
        | ((b[1] as u64) << 8)
        | (b[0] as u64);
    return ulong;
}

fn main() {
    // build_path("/Users/hakkabara/test/mnt/images/DC");
    let buffer = read_bootstat(&PathBuf::from(
        "/Users/hakkabara/code/extract-bootstat/example/bootstat.dat",
    ));
    get_information(buffer);
}
