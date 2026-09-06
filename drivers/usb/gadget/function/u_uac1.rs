//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/gadget/function/u_uac1.h
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
//
// u_uac1.h - Utility definitions for UAC1 function
//
// Copyright (C) 2016 Ruslan Bilovol <ruslan.bilovol@gmail.com>
//

pub const UAC1_OUT_EP_MAX_PACKET_SIZE: c_int = 200;
pub const UAC1_DEF_CCHMASK: c_uint = 0x3;
pub const UAC1_DEF_CSRATE: c_int = 48000;
pub const UAC1_DEF_CSSIZE: c_int = 2;
pub const UAC1_DEF_PCHMASK: c_uint = 0x3;
pub const UAC1_DEF_PSRATE: c_int = 48000;
pub const UAC1_DEF_PSSIZE: c_int = 2;
pub const UAC1_DEF_REQ_NUM: c_int = 2;
pub const UAC1_DEF_INT_REQ_NUM: c_int = 10;
pub const UAC1_DEF_MUTE_PRESENT: c_int = 1;
pub const UAC1_DEF_VOLUME_PRESENT: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct f_uac1_opts {
    pub func_inst: usb_function_instance,
    pub c_chmask: c_int,
    pub c_srates: [c_int; UAC_MAX_RATES],
    pub c_ssize: c_int,
    pub p_chmask: c_int,
    pub p_srates: [c_int; UAC_MAX_RATES],
    pub p_ssize: c_int,
    pub p_mute_present: bool,
    pub p_volume_present: bool,
    pub p_volume_min: i16,
    pub p_volume_max: i16,
    pub p_volume_res: i16,
    pub c_mute_present: bool,
    pub c_volume_present: bool,
    pub c_volume_min: i16,
    pub c_volume_max: i16,
    pub c_volume_res: i16,
    pub req_number: c_int,
    pub bound:1: unsigned,
    pub function_name: [c_char; USB_MAX_STRING_LEN],
    pub p_it_name: [c_char; USB_MAX_STRING_LEN],
    pub p_it_ch_name: [c_char; USB_MAX_STRING_LEN],
    pub p_ot_name: [c_char; USB_MAX_STRING_LEN],
    pub p_fu_vol_name: [c_char; USB_MAX_STRING_LEN],
    pub c_it_name: [c_char; USB_MAX_STRING_LEN],
    pub c_it_ch_name: [c_char; USB_MAX_STRING_LEN],
    pub c_ot_name: [c_char; USB_MAX_STRING_LEN],
    pub c_fu_vol_name: [c_char; USB_MAX_STRING_LEN],
    pub lock: mutex,
    pub refcnt: c_int,
}
