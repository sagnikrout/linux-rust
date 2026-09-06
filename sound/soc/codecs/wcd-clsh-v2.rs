//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/wcd-clsh-v2.h
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
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wcd_clsh_event {
    WCD_CLSH_EVENT_PRE_DAC = 1,
    WCD_CLSH_EVENT_POST_PA,
}

//
// Basic states for Class H state machine.
// represented as a bit mask within a u8 data type
// bit 0: EAR mode
// bit 1: HPH Left mode
// bit 2: HPH Right mode
// bit 3: Lineout mode
//
pub const WCD_CLSH_STATE_IDLE: c_int = 0;

pub const WCD_CLSH_STATE_MAX: c_int = 4;
pub const WCD_CLSH_V3_STATE_MAX: c_int = 5;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wcd_clsh_mode {
    CLS_H_NORMAL = 0, /* Class-H Default */
    CLS_H_HIFI, /* Class-H HiFi */
    CLS_H_LP, /* Class-H Low Power */
    CLS_AB, /* Class-AB */
    CLS_H_LOHIFI, /* LoHIFI */
    CLS_H_ULP, /* Ultra Low power */
    CLS_AB_HIFI, /* Class-AB */
    CLS_AB_LP, /* Class-AB Low Power */
    CLS_AB_LOHIFI, /* Class-AB Low HIFI */
    CLS_NONE, /* None of the above modes */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wcd_codec_version {
    WCD9335  = 0,
    WCD934X  = 1,
// New CLSH after this
    WCD937X  = 2,
    WCD938X  = 3,
    WCD939X  = 4,
}

extern "C" {
    pub fn wcd_clsh_ctrl_free(ctrl: *mut wcd_clsh_ctrl);
}
extern "C" {
    pub fn wcd_clsh_ctrl_get_state(ctrl: *mut wcd_clsh_ctrl) -> c_int;
}
