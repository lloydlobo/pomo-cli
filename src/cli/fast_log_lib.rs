#![allow(unused)]
use fast_log::Config;
use log::{
    error,
    info,
    warn,
};
use miette::ErrReport;
//use fast_log::init_log;

pub trait Log {
    fn init_log(self) -> miette::Result<Config, ErrReport>;
    fn error(self, msg: &str);
    fn warn(self, msg: &str);
    fn info(self, msg: &str);
    fn debug(self, msg: &str);
}

/// * `Config` - the `fast_log` Config.
/// * `Config::file()` - add a FileAppender.
/// * `Config::chan_len()` - if none=> unbounded() channel,if Some =>  bounded(len) channel.
#[derive(Debug)]
pub struct FastLog {
    pub cfg: Config,
}

impl Log for FastLog {
    fn init_log(self) -> miette::Result<Config, ErrReport> {
        Ok(self.cfg.file("log.txt").chan_len(None))
    }

    fn error(self, msg: &str) {
        self.use_log_console(msg)
    }

    fn warn(self, msg: &str) {
        self.use_log_console(msg)
    }

    fn info(self, msg: &str) {
        self.use_log_console(msg)
    }

    fn debug(self, msg: &str) {
        self.use_log_console(msg)
    }
}

impl FastLog {
    pub fn new() -> Self {
        Self { cfg: Config::new() }
    }

    pub fn use_file_chan_perf(self) {
        fast_log::init(self.cfg.file("target/test.log").chan_len(Some(100000))).unwrap();
        log::info!("Commencing yak shaving{}", 0);
    }

    pub fn use_log_console(self, msg: &str) {
        fast_log::init(self.cfg.console().chan_len(Some(100000))).unwrap();
        let msg = format!("Commencing yak shaving: {}", msg);
        log::info!("{}{}", msg, 0);
    }

    pub fn use_log_console_print(self) {
        fast_log::init(self.cfg.console().chan_len(Some(100000))).unwrap();
        fast_log::print("Commencing print\n".into());
    }

    pub fn use_log_file(self) {
        fast_log::init(self.cfg.file("target/test.log").chan_len(Some(100000))).unwrap();
        log::info!("Commencing yak shaving{}", 0);
        info!("Commencing yak shaving");
    }
}
