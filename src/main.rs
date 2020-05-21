extern crate x11;


use std::ptr;
use std::ffi::CString;

use x11::xlib::*;

unsafe fn str_to_cstr(x: &str) -> *const i8 {
    CString::new(x).expect("Failed to create CString.").as_ptr()
}

fn main() {
    let mut dpy: *mut Display = unsafe { XOpenDisplay(ptr::null_mut()) }; 

    if dpy == ptr::null_mut() {
        panic!("Can\'t start X server!");
    }

    unsafe {
        XGrabKey(dpy, XKeysymToKeycode(dpy, XStringToKeysym(str_to_cstr("F1"))) as i32, Mod1Mask,
            XDefaultRootWindow(dpy), 1, GrabModeAsync, GrabModeAsync);
        XGrabButton(dpy, 1, Mod1Mask, XDefaultRootWindow(dpy), 1,
            (ButtonPressMask|ButtonReleaseMask|PointerMotionMask) as u32, GrabModeAsync, GrabModeAsync, 0, 0);
        XGrabButton(dpy, 3, Mod1Mask, XDefaultRootWindow(dpy), 1,
            (ButtonPressMask|ButtonReleaseMask|PointerMotionMask) as u32, GrabModeAsync, GrabModeAsync, 0, 0);
    }
}
