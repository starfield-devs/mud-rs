#[macro_export]
macro_rules! log_pretty {
    ($level:ident, $pretty:expr, $val:expr) => {
        if $pretty {
            log::$level!("{:#?}", $val);
        } else {
            log::$level!("{:?}", $val);
        }
    };

    ($level:ident, $pretty:expr, $fmt:expr, $($arg:tt)*) => {
        log::$level!($fmt, $($arg)*);
    };
}

#[macro_export]
macro_rules! trace_p {
    ($pretty:expr, $($arg:tt)*) => {
        $crate::log_pretty!(trace, $pretty, $($arg)*);
    };
}

#[macro_export]
macro_rules! debug_p {
    ($pretty:expr, $($arg:tt)*) => {
        $crate::log_pretty!(debug, $pretty, $($arg)*);
    };
}

#[macro_export]
macro_rules! info_p {
    ($pretty:expr, $($arg:tt)*) => {
        $crate::log_pretty!(info, $pretty, $($arg)*);
    };
}

#[macro_export]
macro_rules! warn_p {
    ($pretty:expr, $($arg:tt)*) => {
        $crate::log_pretty!(warn, $pretty, $($arg)*);
    };
}

#[macro_export]
macro_rules! error_p {
    ($pretty:expr, $($arg:tt)*) => {
        $crate::log_pretty!(error, $pretty, $($arg)*);
    };
}
