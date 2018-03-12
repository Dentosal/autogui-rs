use winapi::um::winuser;
use winapi::ctypes::c_int;

use crate::Position;

/// Moves mouse, using Win32 SetCursorPos() function
pub(super) fn mouse_move(p: Position) {
    let success = 0 != unsafe {
        winuser::SetCursorPos(p.x as c_int, p.y as c_int)
    };
    assert!(success, "Could not move mouse");
}


// Mouse down, using Win32
// pub(super) fb mouse_down(x, y, button) {

// }
//     """Send the mouse down event to Windows by calling the mouse_event() win32
//     function.
//     Args:
//       x (int): The x position of the mouse event.
//       y (int): The y position of the mouse event.
//       button (str): The mouse button, either 'left', 'middle', or 'right'
//     Returns:
//       None
//     """
//     if button == 'left':
//         try:
//             _sendMouseEvent(MOUSEEVENTF_LEFTDOWN, x, y)
//         except (PermissionError, OSError): # TODO: We need to figure out how to prevent these errors, see https://github.com/asweigart/pyautogui/issues/60
//             pass
//     elif button == 'middle':
//         try:
//             _sendMouseEvent(MOUSEEVENTF_MIDDLEDOWN, x, y)
//         except (PermissionError, OSError): # TODO: We need to figure out how to prevent these errors, see https://github.com/asweigart/pyautogui/issues/60
//             pass
//     elif button == 'right':
//         try:
//             _sendMouseEvent(MOUSEEVENTF_RIGHTDOWN, x, y)
//         except (PermissionError, OSError): # TODO: We need to figure out how to prevent these errors, see https://github.com/asweigart/pyautogui/issues/60
//             pass
//     else:
//         assert False, "button argument not in ('left', 'middle', 'right')"
