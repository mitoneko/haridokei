use log::info;

fn main() {
    systemd_journal_logger::JournalLog::new()
        .unwrap()
        .with_syslog_identifier("haridokei".to_string())
        .install()
        .unwrap();
    log::set_max_level(log::LevelFilter::Info);

    info!("Hello haridokei");
}
