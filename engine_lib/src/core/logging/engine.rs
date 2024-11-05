#[allow(unused_macros)]
macro_rules! trace {
    ($($arg:tt)+) => (log::trace!(target: "ENGINE", $($arg)+));
}
#[allow(unused_imports)]
pub(crate) use trace;

#[allow(unused_macros)]
macro_rules! debug {
    ($($arg:tt)+) => (log::debug!(target: "ENGINE", $($arg)+));
}
#[allow(unused_imports)]
pub(crate) use debug;

#[allow(unused_macros)]
macro_rules! info {
    ($($arg:tt)+) => (log::info!(target: "ENGINE", $($arg)+));
}
#[allow(unused_imports)]
pub(crate) use info;

#[allow(unused_macros)]
macro_rules! warning {
    ($($arg:tt)+) => (log::warn!(target: "ENGINE", $($arg)+));
}
#[allow(unused_imports)]
pub(crate) use warning;

#[allow(unused_macros)]
macro_rules! error {
    ($($arg:tt)+) => (log::error!(target: "ENGINE", $($arg)+));
}
#[allow(unused_imports)]
pub(crate) use error;
