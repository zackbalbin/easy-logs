use std::{io::Write, fs};
use colored::{Colorize, ColoredString};


/// Easy Logger
pub struct Log {
    path: Option<String>,
}

impl Log {
    /// Create a new logger.
    pub fn new(output_to_file: bool)  -> Log {
        let date: String = chrono::Local::now().format("%Y-%m-%d_%H:%M:%S").to_string();
        if output_to_file {
            fs::create_dir_all("logs").unwrap();
            let path: Option<String> = Some(format!("logs/{}.log", date));
            fs::File::create(path.clone().unwrap()).unwrap();
            let log: Log = Log {
                path: path.clone(),
            };
            return log;
        } else {
            let log: Log = Log {
                path: None,
            };
            return log;
        }
    }

    /// Log an info message.
    pub fn info(&self, message: &str) {
        let date: String = chrono::Utc::now().format("%m-%d-%Y [%H:%M:%S.%f]").to_string();
        let info_text: ColoredString = "[INFO] ".green().bold();
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
        println!("{}", &message_text);
    }

    /// Log a warning message.
    pub fn warn(&self, message: &str) {
        let date: String = chrono::Utc::now().format("%m-%d-%Y [%H:%M:%S.%f]").to_string();
        let info_text: ColoredString = "[WARN] ".yellow().bold();
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
        println!("{}", &message_text);
    }

    /// Log an error message.
    pub fn error(&self, message: &str) {
        let date: String = chrono::Utc::now().format("%m-%d-%Y [%H:%M:%S.%f]").to_string();
        let info_text: ColoredString = "[ERROR] ".red().bold();
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
        println!("{}", &message_text);
    }

    /// Log a debug message.
    pub fn debug(&self, message: &str) {
        let date: String = chrono::Utc::now().format("%m-%d-%Y [%H:%M:%S.%f]").to_string();
        let info_text: ColoredString = "[DEBUG] ".blue().bold();
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
        println!("{}", &message_text);
    }
}