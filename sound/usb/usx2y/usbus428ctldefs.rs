//! Automatically rewritten from C Header to Rust Module
//! Source: sound/usb/usx2y/usbus428ctldefs.h
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
// Copyright (c) 2003 by Karsten Wiese <annabellesgarden@yahoo.de>
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum E_IN84 {
    E_FADER_0 = 0,
    E_FADER_1,
    E_FADER_2,
    E_FADER_3,
    E_FADER_4,
    E_FADER_5,
    E_FADER_6,
    E_FADER_7,
    E_FADER_M,
    E_TRANSPORT,
    E_MODIFIER = 10,
    E_FILTER_SELECT,
    E_SELECT,
    E_MUTE,

    E_SWITCH   = 15,
    E_WHEEL_GAIN,
    E_WHEEL_FREQ,
    E_WHEEL_Q,
    E_WHEEL_PAN,
    E_WHEEL    = 20
}

pub const T_RECORD: c_int = 1;
pub const T_PLAY: c_int = 2;
pub const T_STOP: c_int = 4;
pub const T_F_FWD: c_int = 8;
pub const T_REW: c_uint = 0x10;
pub const T_SOLO: c_uint = 0x20;
pub const T_REC: c_uint = 0x40;
pub const T_NULL: c_uint = 0x80;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct us428_ctls {
    pub fader: [c_uchar; 9],
    pub transport: c_uchar,
    pub modifier: c_uchar,
    pub filters_elect: c_uchar,
    pub select: c_uchar,
    pub mute: c_uchar,
    pub unknown: c_uchar,
    pub wswitch: c_uchar,
    pub wheel: [c_uchar; 5],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct us428_set_byte {
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usx2y_volume {
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct us428_lights {
    pub light: [us428_set_byte; 7],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct us428_p4out {
    pub type: c_char,
    pub vol: usx2y_volume,
    pub lights: us428_lights,
    pub val: },
}

pub const N_US428_CTL_BUFS: c_int = 16;
pub const N_US428_P4OUT_BUFS: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct us428ctls_sharedmem {
    pub ctl_snapshot: [us428_ctls; N_US428_CTL_BUFS],
    pub ctl_snapshot_differs_at: [c_int; N_US428_CTL_BUFS],
    pub ctl_snapshot_red: int ctl_snapshot_last,,
    pub p4out: [us428_p4out; N_US428_P4OUT_BUFS],
    pub p4out_sent: int p4out_last,,
}
