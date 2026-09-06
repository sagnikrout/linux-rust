//! Automatically rewritten from C Header to Rust Module
//! Source: sound/usb/usx2y/us122l.h
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct us122l {
    pub dev: *mut usb_device,
    pub card_index: c_int,
    pub stride: c_int,
    pub sk: usb_stream_kernel,
    pub mutex: mutex,
    pub first: *mut file,
    pub second_periods_polled: c_uint,
    pub master: *mut file,
    pub slave: *mut file,
    pub midi_list: list_head,
    pub is_us144: bool,
}

pub const USB_ID_US122L: c_uint = 0x800E;
pub const USB_ID_US144: c_uint = 0x800F;
pub const USB_ID_US122MKII: c_uint = 0x8021;
pub const USB_ID_US144MKII: c_uint = 0x8020;
