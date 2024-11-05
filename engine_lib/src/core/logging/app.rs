#[allow(unused_macros)]
#[macro_export]
macro_rules! trace {
    ($($arg:tt)+) => ($crate::external::trace!(target: "APP", $($arg)+));
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! debug {
    ($($arg:tt)+) => ($crate::external::debug!(target: "APP", $($arg)+));
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! info {
    ($($arg:tt)+) => ($crate::external::info!(target: "APP", $($arg)+));
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! warn {
    ($($arg:tt)+) => ($crate::external::warn!(target: "APP", $($arg)+));
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! error {
    ($($arg:tt)+) => ($crate::external::error!(target: "APP", $($arg)+));
}
