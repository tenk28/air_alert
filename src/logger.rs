use flexi_logger::*;

const LOGS_DIRECTORY: &str = "logs";

pub fn init_logger() {
    let file_spec: FileSpec = FileSpec::default().directory(LOGS_DIRECTORY);

    Logger::try_with_env_or_str("info")
        .unwrap()
        .log_to_file(file_spec)
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
