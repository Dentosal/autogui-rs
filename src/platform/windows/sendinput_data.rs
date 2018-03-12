#![allow(non_snake_case)]

use std::mem::size_of;
use winapi::ctypes::{c_long, c_ulong, c_ushort};

/// # Win32 MOUSEINPUT
#[repr(C)]
struct Win32MouseInput {
    dx:             c_long,
    dy:             c_long,
    mouseData:      c_ulong,
    dwFlags:        c_ulong,
    time:           c_ulong,
    dwExtraInfo:    *const c_ulong
}

/// # Win32 KEYBDINPUT
#[repr(C)]
struct Win32KeyboardInput {
    wVk:            c_ushort,
    wScan:          c_ushort,
    dwFlags:        c_ulong,
    time:           c_ulong,
    dwExtraInfo:    *const c_ulong
}

/// # Win32 HARDWAREINPUT
#[repr(C)]
struct Win32HardwareInput {
    uMsg:           c_ulong,
    wParamL:        c_ushort,
    wParamH:        c_ulong
}

/// # Win32 INPUT
#[repr(C)]
struct Win32Input {
    // union, variants:
    // Win32MouseInput, Win32KeyboardInput, Win32HardwareInput
    typeTag: c_ulong,
    data: [u8; size_of::<Win32MouseInput>()] // largest member
}
