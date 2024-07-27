use flexi_logger::*;
use std::{fs, time::Duration};


pub fn init_logger(logs_dir: &str) {
    let file_spec: FileSpec = FileSpec::default().directory(logs_dir);
    let _ = fs::create_dir_all(logs_dir);

    Logger::try_with_env_or_str("info")
        .unwrap()
        .log_to_file(file_spec)
        .write_mode(WriteMode::BufferAndFlushWith(
            DEFAULT_BUFFER_CAPACITY,
            Duration::from_secs(10),
        ))
        .duplicate_to_stdout(Duplicate::Info)
        .duplicate_to_stdout(Duplicate::Warn)
        .duplicate_to_stderr(Duplicate::Error)
        .rotate(
            Criterion::Age(Age::Day),
            Naming::TimestampsCustomFormat {
                current_infix: None,
                format: "%d-%m-%Y",
            },
            Cleanup::KeepLogFiles(7),
        )
        .format(|writer, now, record| {
            writeln!(
                writer,
                "[{}] [{}] [{}:{}] {}",
                now.now(),
                record.level(),
                record.file().unwrap_or("<unknown>"),
                record.line().unwrap_or(0),
                &record.args()
            )
        })
        .start()
        .unwrap_or_else(|e| panic!("Logger initialization failed with {}", e));
}
