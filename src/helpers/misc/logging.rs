use std::io::{Write, Result, Error, ErrorKind};
use flexi_logger::{DeferredNow, Record};
use log::Level;
use colored::Colorize;
use time::{macros::format_description, OffsetDateTime};

pub fn format_colored_log(w: &mut dyn Write, _d: &mut DeferredNow, record: &Record) -> Result<()> {
    let log_message = format!(
        "[{}] [{}]: {}",
        record.level(),
        OffsetDateTime::now_utc()
            .format(
                format_description!(
                    "[hour]:[minute]:[second] [month]/[day]/[year]"
                )
            )
            .map_err(|err| Error::new(ErrorKind::Other, err.to_string()))?,
        &record.args()
    );

    write!(w, "{}", match record.level() {
        Level::Error => log_message.red(),
        Level::Warn => log_message.yellow(),
        Level::Info => log_message.cyan(),
        Level::Debug => log_message.green(),
        Level::Trace => log_message.purple()
    })
}
