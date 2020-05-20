extern crate x11;

use std::ptr;
use x11::xlib;

fn main() {
    let mut dpy = unsafe { xlib::XOpenDisplay(ptr::null_mut()) };


}
