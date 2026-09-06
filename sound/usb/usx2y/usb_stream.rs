//! Automatically rewritten from C Header to Rust Module
//! Source: sound/usb/usx2y/usb_stream.h
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

pub const USB_STREAM_NURBS: c_int = 4;
pub const USB_STREAM_URBDEPTH: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_stream_kernel {
    pub s: *mut usb_stream,
    pub dev: *mut usb_device,
    pub write_page: *mut c_void,
    pub n_o_ps: c_uint,
    pub inurb: [*mut urb; USB_STREAM_NURBS],
    pub idle_inurb: *mut urb,
    pub completed_inurb: *mut urb,
    pub outurb: [*mut urb; USB_STREAM_NURBS],
    pub idle_outurb: *mut urb,
    pub completed_outurb: *mut urb,
    pub i_urb: *mut urb,
    pub iso_frame_balance: c_int,
    pub sleep: wait_queue_head_t,
    pub out_phase: c_uint,
    pub out_phase_peeked: c_uint,
    pub freqn: c_uint,
}

extern "C" {
    pub fn usb_stream_free(sk: *mut usb_stream_kernel);
}
extern "C" {
    pub fn usb_stream_start(sk: *mut usb_stream_kernel) -> c_int;
}
extern "C" {
    pub fn usb_stream_stop(sk: *mut usb_stream_kernel);
}
