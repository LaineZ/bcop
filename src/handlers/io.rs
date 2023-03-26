use sciter::dispatch_script_call;

pub struct Io;

impl Io {
    fn read(&self, filename: String) -> String {
        std::fs::read_to_string(filename).unwrap_or_default()
    }

    fn write(&self, filename: String, contents: String) -> bool {
        std::fs::write(filename, contents).is_ok()
    }

    fn delete(&self, filename: String) -> bool {
        std::fs::remove_file(filename).is_ok()
    }
}

impl sciter::EventHandler for Io {
    dispatch_script_call! {
        fn read(String);
        fn write(String, String);
        fn delete(String);
    }
}
