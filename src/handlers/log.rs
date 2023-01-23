use sciter::{dispatch_script_call, Value};

pub struct Log;

impl Log {
    fn log_info(&self, log: Value) {
        log::info!("{}", log);
    }

    fn log_warn(&self, log: Value) {
        log::warn!("{}", log);
    }

    fn log_err(&self, log: Value) {
        log::error!("{}", log);
    }

    fn log_debug(&self, log: Value) {
        log::debug!("{}", log);
    }
}

impl sciter::EventHandler for Log {
    dispatch_script_call! {
      fn log_info(Value);
      fn log_warn(Value);
      fn log_err(Value);
      fn log_debug(Value);
    }
}
