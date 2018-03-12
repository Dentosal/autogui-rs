use winapi::ctypes::c_ulong;

/// # Win32 POINT
/// [MSDN](https://msdn.microsoft.com/en-us/library/windows/desktop/dd162805(v=vs.85).aspx)
#[repr(C)]
pub(super) struct Win32Point {
    x: c_ulong,
    y: c_ulong
}
impl Win32Point {
    pub fn new(x: u32, y: u32) -> Win32Point {
        Win32Point { x, y }
    }
}
