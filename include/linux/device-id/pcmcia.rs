//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/device-id/pcmcia.h
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

pub type kernel_ulong_t = c_ulong;

// PCMCIA
pub const PCMCIA_DEV_ID_MATCH_MANF_ID: c_uint = 0x0001;
pub const PCMCIA_DEV_ID_MATCH_CARD_ID: c_uint = 0x0002;
pub const PCMCIA_DEV_ID_MATCH_FUNC_ID: c_uint = 0x0004;
pub const PCMCIA_DEV_ID_MATCH_FUNCTION: c_uint = 0x0008;
pub const PCMCIA_DEV_ID_MATCH_PROD_ID1: c_uint = 0x0010;
pub const PCMCIA_DEV_ID_MATCH_PROD_ID2: c_uint = 0x0020;
pub const PCMCIA_DEV_ID_MATCH_PROD_ID3: c_uint = 0x0040;
pub const PCMCIA_DEV_ID_MATCH_PROD_ID4: c_uint = 0x0080;
pub const PCMCIA_DEV_ID_MATCH_DEVICE_NO: c_uint = 0x0100;
pub const PCMCIA_DEV_ID_MATCH_FAKE_CIS: c_uint = 0x0200;
pub const PCMCIA_DEV_ID_MATCH_ANONYMOUS: c_uint = 0x0400;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcmcia_device_id {
    pub match_flags: __u16,
    pub manf_id: __u16,
    pub card_id: __u16,
    pub func_id: __u8,
// for real multi-function devices
    pub function: __u8,
// for pseudo multi-function devices
    pub device_no: __u8,
    pub prod_id_hash: [__u32; 4],
// not matched against in kernelspace
    pub prod_id: [*const *const c_char; 4],
// not matched against
    pub driver_info: kernel_ulong_t,
    pub cisfile: *mut *mut c_char,
}
