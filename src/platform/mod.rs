#[cfg(target_os = "linux")]     pub(crate) mod linux;
#[cfg(target_os = "macos")]     pub(crate) mod macos;
#[cfg(target_os = "windows")]   pub(crate) mod windows;

#[cfg(target_os = "linux")]     pub(crate) use self::linux   as current;
#[cfg(target_os = "macos")]     pub(crate) use self::macos   as current;
#[cfg(target_os = "windows")]   pub(crate) use self::windows as current;