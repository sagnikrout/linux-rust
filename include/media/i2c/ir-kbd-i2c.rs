//! Automatically rewritten from C Header to Rust Module
//! Source: include/media/i2c/ir-kbd-i2c.h
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
pub struct IR_i2c {
    pub ir_codes: *mut c_char,
    pub c: *mut i2c_client,
    pub rc: *mut rc_dev,
// Used to avoid fast repeating
    pub old: c_uchar,
    pub /: *mut *mut u32 polling_interval; / in ms,
    pub work: delayed_work,
    pub phys: [c_char; 32],
    pub toggle): *mut *mut u32 scancode, u8,
// tx
    pub tx_c: *mut i2c_client,
    pub /: *mut *mut mutex lock; / do not poll Rx during Tx,
    pub carrier: c_uint,
    pub duty_cycle: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ir_kbd_get_key_fn {
    IR_KBD_GET_KEY_CUSTOM = 0,
    IR_KBD_GET_KEY_PIXELVIEW,
    IR_KBD_GET_KEY_HAUP,
    IR_KBD_GET_KEY_KNC1,
    IR_KBD_GET_KEY_GENIATECH,
    IR_KBD_GET_KEY_FUSIONHDTV,
    IR_KBD_GET_KEY_HAUP_XVR,
    IR_KBD_GET_KEY_AVERMEDIA_CARDBUS,
}

// Can be passed when instantiating an ir_video i2c device
#[repr(C)]
#[derive(Copy, Clone)]
pub struct IR_i2c_init_data {
    pub ir_codes: *mut c_char,
    pub name: *const c_char,
    pub /: *mut *mut u64 type; / RC_PROTO_BIT_RC5, etc,
    pub /: *mut *mut u32 polling_interval; / 0 means DEFAULT_POLLING_INTERVAL,
//
// Specify either a function pointer or a value indicating one of
// ir_kbd_i2c's internal get_key functions
//
    pub toggle): *mut *mut u32 scancode, u8,
    pub internal_get_key_func: ir_kbd_get_key_fn,
    pub rc_dev: *mut rc_dev,
}
