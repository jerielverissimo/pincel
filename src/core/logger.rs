use num_derive::FromPrimitive;

#[derive(Copy, Clone, FromPrimitive, PartialEq, Eq, Debug, PartialOrd)]
pub enum LogLevel {
    /// Fatal log level, should be used to stop the application when hit.
    Fatal = 0,
    /// Error log level, should be used to indicate critical runtime problems that cause the application to run improperly or not at all.
    Error = 1,
    /// Warning log level, should be used to indicate non-critial problems with the application that cause it to run suboptimally.
    Warn = 2,
    /// Info log level, should be used for non-erronuous informational purposes.
    Info = 3,
    /// Debug log level, should be used for debugging purposes.
    Debug = 4,
    /// Trace log level, should be used for verbose debugging purposes.
    Trace = 5,
}

fn platform_console_write(message: impl Into<String>, colour: usize) {
    // FATAL,ERROR,WARN,INFO,DEBUG,TRACE
    let colour_strings = ["0;41", "1;31", "1;33", "1;32", "1;34", "1;30"];
    print!("\x1B[{}m{}\x1B[0m", colour_strings[colour], message.into());
}

fn platform_console_write_error(message: impl Into<String>, colour: usize) {
    // FATAL,ERROR,WARN,INFO,DEBUG,TRACE
    let colour_strings = ["0;41", "1;31", "1;33", "1;32", "1;34", "1;30"];
    print!("\x1B[{}m{}\x1B[0m", colour_strings[colour], message.into());
}

pub fn log_output(level: LogLevel, message: impl Into<String>) {
    // TODO: These string operations are all pretty slow. This needs to be
    // moved to another thread eventually, along with the file writes, to
    // avoid slowing things down while the engine is trying to run.
    let level_strings = [
        "[FATAL]: ",
        "[ERROR]: ",
        "[WARN]:  ",
        "[INFO]:  ",
        "[DEBUG]: ",
        "[TRACE]: ",
    ];
    let is_error = level < LogLevel::Warn;

    // Prepend log level to message.
    let out_message = format!("{}{}\n", level_strings[level as usize], message.into());

    // Print accordingly
    if is_error {
        platform_console_write_error(out_message, level as usize);
    } else {
        platform_console_write(out_message, level as usize);
    }
}
