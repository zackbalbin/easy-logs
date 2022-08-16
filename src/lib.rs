use std::{io::Write, fs};
use colored::{Colorize, ColoredString};


/// Easy Logger
pub struct Log {
    path: Option<String>,
}

impl Log {
    /// Create a new logger.
    pub fn new()  -> Log {
        let log: Log = Log {
            path: None,
        };
        return log;
    }

    /// Update logger to output logs to a file.
    pub fn output_to_file(&mut self) {
        Self::output_to_file_impl(self);
    }

    fn output_to_file_impl(&mut self) {
        let date: String = chrono::Local::now().format("%Y-%m-%d_%H:%M:%S").to_string();
        fs::create_dir_all("logs").unwrap();
        let file_path: Option<String> = Some(format!("logs/{}.log", date));
        fs::File::create(file_path.clone().unwrap()).unwrap();
        self.path = file_path;
    }

    /// Log an info message.
    pub fn info(&self, message: &str) {
        let date: String = chrono::Utc::now().format("%m-%d-%Y [%H:%M:%S.%f]").to_string();
        let info_text: &str = "[INFO] ";
        let message_text: String = format!("{} {} {}", date, info_text, message);
        if self.path.is_some() {
            let mut file = fs::OpenOptions::new()
                .write(true)
                .append(true)
                .open(self.path.clone().unwrap())
                .unwrap();
            let log_text: String = format!("{}\n", message_text);
            file.write_all(log_text.as_bytes()).unwrap();
        }
        let info_text: ColoredString = info_text.green().bold();
        let message_text: String = format!("{} {} {}", date, info_text, message);
        println!("{}", &message_text);
    }

    /// Log a warning message.
    pub fn warn(&self, message: &str) {
        let date: String = chrono::Utc::now().format("%m-%d-%Y [%H:%M:%S.%f]").to_string();
        let info_text: &str = "[WARN] ";
        let message_text: String = format!("{} {} {}", date, info_text, message);
        if self.path.is_some() {
            let mut file = fs::OpenOptions::new()
                .write(true)
                .append(true)
                .open(self.path.clone().unwrap())
                .unwrap();
            let log_text: String = format!("{}\n", message_text);
            file.write_all(log_text.as_bytes()).unwrap();
        }
        let info_text: ColoredString = info_text.yellow().bold();
        let message_text: String = format!("{} {} {}", date, info_text, message);
        println!("{}", &message_text);
    }

    /// Log an error message.
    pub fn error(&self, message: &str) {
        let date: String = chrono::Utc::now().format("%m-%d-%Y [%H:%M:%S.%f]").to_string();
        let info_text: &str = "[ERROR] ";
        let message_text: String = format!("{} {} {}", date, info_text, message);
        if self.path.is_some() {
            let mut file = fs::OpenOptions::new()
                .write(true)
                .append(true)
                .open(self.path.clone().unwrap())
                .unwrap();
            let log_text: String = format!("{}\n", message_text);
            file.write_all(log_text.as_bytes()).unwrap();
        }
        let info_text: ColoredString = info_text.red().bold();
        let message_text: String = format!("{} {} {}", date, info_text, message);
        println!("{}", &message_text);
    }

    /// Log a debug message.
    pub fn debug(&self, message: &str) {
        let date: String = chrono::Utc::now().format("%m-%d-%Y [%H:%M:%S.%f]").to_string();
        let info_text: &str = "[DEBUG] ";
        let message_text: String = format!("{} {} {}", date, info_text, message);
        if self.path.is_some() {
            let mut file = fs::OpenOptions::new()
                .write(true)
                .append(true)
                .open(self.path.clone().unwrap())
                .unwrap();
            let log_text: String = format!("{}\n", message_text);
            file.write_all(log_text.as_bytes()).unwrap();
        }
        let info_text: ColoredString = info_text.blue().bold();
        let message_text: String = format!("{} {} {}", date, info_text, message);
        println!("{}", &message_text);
    }
}