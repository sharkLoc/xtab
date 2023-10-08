use std::io::Write;
use chrono::Local;
use env_logger::{Builder,fmt::Color};
use log::{LevelFilter,Level};


pub fn logs() {
    let mut builder = Builder::from_default_env();
    builder.format(|buf, record| {
        let mut style = buf.style();
        match record.level() {
            Level::Error => {
                style.set_color(Color::Red).set_bold(true);
            }
            Level::Warn => {
                style.set_color(Color::Yellow).set_bold(true);
            }
            Level::Info => {
                style.set_color(Color::Green).set_bold(true);
            }
            Level::Debug => {
                style.set_color(Color::Blue).set_bold(true);
            }
            Level::Trace => {
                style.set_color(Color::Magenta).set_bold(true);
            }
        }
        writeln!(buf,
            "[{} {} - {}] {}",
            Local::now().format("%Y-%m-%dT%H:%M:%S"),
            style.value(record.level()),
            buf.style().set_color(Color::Rgb(90, 150, 150)).value(record.target()),
            record.args()
        )
    })
    .filter(None, LevelFilter::Info)
    .init();
}