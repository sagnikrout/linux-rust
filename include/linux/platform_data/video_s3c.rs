//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/video_s3c.h
#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use core::ffi::*;

// --- Linux Kernel Primitives Prelude ---
pub type uid_t = u32;
pub type gid_t = u32;
pub type uid16_t = u16;
pub type gid16_t = u16;
pub type pid_t = i32;
pub type mode_t = u32;
pub type umode_t = u16;
pub type nlink_t = u32;
pub type off_t = i64;
pub type loff_t = i64;
pub type dev_t = u32;
pub type ino_t = u64;
pub type size_t = usize;
pub type ssize_t = isize;
pub type uintptr_t = usize;
pub type intptr_t = isize;
pub type ptrdiff_t = isize;
pub type clockid_t = i32;
pub type timer_t = i32;
pub type time64_t = i64;
pub type atomic_t = core::sync::atomic::AtomicI32;
pub type atomic64_t = core::sync::atomic::AtomicI64;
// ---------------------------------------


// SPDX-License-Identifier: GPL-2.0
// S3C_FB_MAX_WIN
// Set to the maximum number of windows that any of the supported hardware
// can use. Since the platform data uses this for an array size, having it
// set to the maximum of any version of the hardware can do is safe.
//

//
// struct s3c_fb_pd_win - per window setup data
// @xres     : The window X size.
// @yres     : The window Y size.
// @virtual_x: The virtual X size.
// @virtual_y: The virtual Y size.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct s3c_fb_pd_win {
    pub default_bpp: c_ushort,
    pub max_bpp: c_ushort,
    pub xres: c_ushort,
    pub yres: c_ushort,
    pub virtual_x: c_ushort,
    pub virtual_y: c_ushort,
}

//
// struct s3c_fb_platdata -  S3C driver platform specific information
// @setup_gpio: Setup the external GPIO pins to the right state to transfer
// the data from the display system to the connected display
// device.
// @vidcon0: The base vidcon0 values to control the panel data format.
// @vidcon1: The base vidcon1 values to control the panel data output.
// @vtiming: Video timing when connected to a RGB type panel.
// @win: The setup data for each hardware window, or NULL for unused.
// @display_mode: The LCD output display mode.
//
// The platform data supplies the video driver with all the information
// it requires to work with the display(s) attached to the machine. It
// controls the initial mode, the number of display windows (0 is always
// the base framebuffer) that are initialised etc.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct s3c_fb_platdata {
    pub (*setup_gpio)(void): *mut c_void,
    pub win: [*mut s3c_fb_pd_win; S3C_FB_MAX_WIN],
    pub vtiming: *mut fb_videomode,
    pub vidcon0: u32,
    pub vidcon1: u32,
}
