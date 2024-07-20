use flexi_logger::{Duplicate, FileSpec, Logger};

const LOGS_DIRECTORY: &str = "logs";

pub fn init_logger() {
    let file_spec: FileSpec = FileSpec::default().directory(LOGS_DIRECTORY);

    Logger::try_with_env_or_str("info")
        .unwrap()
        .log_to_file(file_spec)
        .duplicate_to_stderr(Duplicate::Error)
        .duplicate_to_stdout(Duplicate::Warn)
        .duplicate_to_stdout(Duplicate::Info)
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
