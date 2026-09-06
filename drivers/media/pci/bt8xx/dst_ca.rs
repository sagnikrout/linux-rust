//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/bt8xx/dst_ca.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
pub const RETRIES: c_int = 5;
pub const CA_APP_INFO_ENQUIRY: c_uint = 0x9f8020;
pub const CA_APP_INFO: c_uint = 0x9f8021;
pub const CA_ENTER_MENU: c_uint = 0x9f8022;
pub const CA_INFO_ENQUIRY: c_uint = 0x9f8030;
pub const CA_INFO: c_uint = 0x9f8031;
pub const CA_PMT: c_uint = 0x9f8032;
pub const CA_PMT_REPLY: c_uint = 0x9f8033;
pub const CA_CLOSE_MMI: c_uint = 0x9f8800;
pub const CA_DISPLAY_CONTROL: c_uint = 0x9f8801;
pub const CA_DISPLAY_REPLY: c_uint = 0x9f8802;
pub const CA_TEXT_LAST: c_uint = 0x9f8803;
pub const CA_TEXT_MORE: c_uint = 0x9f8804;
pub const CA_KEYPAD_CONTROL: c_uint = 0x9f8805;
pub const CA_KEYPRESS: c_uint = 0x9f8806;
pub const CA_ENQUIRY: c_uint = 0x9f8807;
pub const CA_ANSWER: c_uint = 0x9f8808;
pub const CA_MENU_LAST: c_uint = 0x9f8809;
pub const CA_MENU_MORE: c_uint = 0x9f880a;
pub const CA_MENU_ANSWER: c_uint = 0x9f880b;
pub const CA_LIST_LAST: c_uint = 0x9f880c;
pub const CA_LIST_MORE: c_uint = 0x9f880d;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dst_ca_private {
    pub dst: *mut dst_state,
    pub dvbdev: *mut dvb_device,
}
