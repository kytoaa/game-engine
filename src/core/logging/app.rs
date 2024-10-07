#[allow(unused_macros)]
#[macro_export]
macro_rules! trace {
    ($($arg:tt)+) => (log::trace!(target: "APP", $($arg)+));
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! debug {
    ($($arg:tt)+) => (log::debug!(target: "APP", $($arg)+));
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! info {
    ($($arg:tt)+) => (log::info!(target: "APP", $($arg)+));
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! warn {
    ($($arg:tt)+) => (log::warn!(target: "APP", $($arg)+));
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! error {
    ($($arg:tt)+) => (log::error!(target: "APP", $($arg)+));
}
