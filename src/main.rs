extern crate x11;

use std::ffi::CString;
use std::mem::zeroed;
use std::ptr;

use x11::xlib::*;

unsafe fn str_to_cstr(x: &str) -> *const i8 {
    CString::new(x).expect("Failed to create CString.").as_ptr()
}

fn max(a: i32, b: i32) -> u32 {
    if a < b {
        a as u32
    } else {
        b as u32
    }
}

fn main() {
    let dpy: *mut Display = unsafe { XOpenDisplay(ptr::null_mut()) };
    let mut attr: XWindowAttributes = unsafe { zeroed() };
    let mut start: XButtonEvent = unsafe { zeroed() };

    if dpy.is_null() {
        panic!("Can\'t start X server!");
    }

    unsafe {
        XGrabKey(
            dpy,
            XKeysymToKeycode(dpy, XStringToKeysym(str_to_cstr("F1"))) as i32,
            Mod1Mask,
            XDefaultRootWindow(dpy),
            1,
            GrabModeAsync,
            GrabModeAsync,
        );
        XGrabButton(
            dpy,
            1,
            Mod1Mask,
            XDefaultRootWindow(dpy),
            1,
            (ButtonPressMask | ButtonReleaseMask | PointerMotionMask) as u32,
            GrabModeAsync,
            GrabModeAsync,
            0,
            0,
        );
        XGrabButton(
            dpy,
            3,
            Mod1Mask,
            XDefaultRootWindow(dpy),
            1,
            (ButtonPressMask | ButtonReleaseMask | PointerMotionMask) as u32,
            GrabModeAsync,
            GrabModeAsync,
            0,
            0,
        );
    }

    start.subwindow = 0;

    let mut ev: XEvent = unsafe { zeroed() };
    loop {
        unsafe {
            XNextEvent(dpy, &mut ev);

            if ev.type_ == KeyPress && ev.key.subwindow != 0 {
                XRaiseWindow(dpy, ev.key.subwindow);
            } else if ev.type_ == ButtonPress && ev.button.subwindow != 0 {
                XGetWindowAttributes(dpy, ev.button.subwindow, &mut attr);
                start = ev.button;
            } else if ev.type_ == MotionNotify && start.subwindow != 0 {
                let xdiff = ev.button.x_root - start.x_root;
                let ydiff = ev.button.y_root - start.y_root;
                XMoveResizeWindow(
                    dpy,
                    start.subwindow,
                    attr.x + {
                        if start.button == 1 {
                            xdiff
                        } else {
                            0
                        }
                    },
                    attr.y + {
                        if start.button == 1 {
                            ydiff
                        } else {
                            0
                        }
                    },
                    max(
                        1,
                        attr.width + {
                            if start.button == 3 {
                                xdiff
                            } else {
                                0
                            }
                        },
                    ),
                    max(
                        1,
                        attr.height + {
                            if start.button == 3 {
                                ydiff
                            } else {
                                0
                            }
                        },
                    ),
                );
            } else if ev.type_ == ButtonRelease {
                start.subwindow = 0;
            }
        }
    }
}
