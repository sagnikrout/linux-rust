//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/video/fbdev/riva/rivafb.h
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

// GGI compatibility macros
pub const NUM_SEQ_REGS: c_uint = 0x05;
pub const NUM_CRT_REGS: c_uint = 0x41;
pub const NUM_GRC_REGS: c_uint = 0x09;
pub const NUM_ATC_REGS: c_uint = 0x15;
// I2C

// holds the state of the VGA core and extended Riva hw state from riva_hw.c.
// From KGI originally.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct riva_regs {
    pub attr: [u8; NUM_ATC_REGS],
    pub crtc: [u8; NUM_CRT_REGS],
    pub gra: [u8; NUM_GRC_REGS],
    pub seq: [u8; NUM_SEQ_REGS],
    pub misc_output: u8,
    pub ext: RIVA_HW_STATE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct riva_i2c_chan {
    pub par: *mut riva_par,
    pub ddc_base: c_ulong,
    pub adapter: i2c_adapter,
    pub algo: i2c_algo_bit_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct riva_par {
    pub /: *mut *mut RIVA_HW_INST riva; / interface to riva_hw.c,
    pub /: *mut *mut u32 pseudo_palette[16]; / default palette,
    pub /: *mut *mut u32 palette[16]; / for Riva128,
    pub /: *mut *mut *mut u8 __iomem ctrl_base; / virtual control register base addr,
    pub /: *mut *mut unsigned dclk_max; / max DCLK,
    pub /: *mut *mut riva_regs initial_state; / initial startup video mode,
    pub current_state: riva_regs,

    pub state: vgastate,

    pub open_lock: mutex,
    pub ref_count: c_uint,
    pub EDID: *mut c_uchar,
    pub Chipset: c_uint,
    pub forceCRTC: c_int,
    pub SecondCRTC: Bool,
    pub FlatPanel: c_int,
    pub pdev: *mut pci_dev,
    pub cursor_reset: c_int,
    pub wc_cookie: c_int,
    pub chan: [riva_i2c_chan; 3],
}

extern "C" {
    pub fn riva_common_setup(: *mut riva_par);
}
extern "C" {
    pub fn riva_get_memlen(: *mut riva_par) -> c_ulong;
}
extern "C" {
    pub fn riva_get_maxdclk(: *mut riva_par) -> c_ulong;
}
extern "C" {
    pub fn riva_delete_i2c_busses(par: *mut riva_par);
}
extern "C" {
    pub fn riva_create_i2c_busses(par: *mut riva_par);
}
extern "C" {
    pub fn riva_probe_i2c_connector(par: *mut riva_par, conn: c_int, out_edid: *mut u8) -> c_int;
}
