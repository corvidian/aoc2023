use super::get_filename;
use simplelog::*;
use std::fs;
use std::path::Path;

pub use simplelog::LevelFilter;

pub fn init_logging(term: LevelFilter, file: LevelFilter) {
    fs::create_dir_all("target/log").expect("Cannot create log directory");
    let filename = get_filename();
    let filename = filename.split_once('.').unwrap().0;
    let mut i = 1;
    while Path::new(&format!("target/log/{filename}_{i}.txt")).exists() {
        i += 1;
    }
    let filename = format!("target/log/{filename}_{i}.txt");
    let config = ConfigBuilder::new()
        .set_time_offset_to_local()
        .expect("Local timezone not found")
        .build();

    CombinedLogger::init(vec![
        TermLogger::new(term, config.clone(), TerminalMode::Mixed, ColorChoice::Auto),
        WriteLogger::new(file, config, fs::File::create(&filename).unwrap()),
    ])
    .unwrap();

    println!("Logs outputting to {filename}")
}
