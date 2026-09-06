//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/phy/freescale/phy-fsl-lynx-core.h
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


// SPDX-License-Identifier: GPL-2.0+
// Copyright 2025-2026 NXP

pub const LYNX_NUM_PLL: c_int = 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lynx_pccr {
    pub offset: c_int,
    pub width: c_int,
    pub shift: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lynx_pll {
    pub priv: *mut lynx_priv,
    pub id: c_int,
    pub refclk_sel: c_int,
    pub frate_sel: c_int,
    pub enabled: bool,
    pub locked: bool,
    pub LANE_MODE_MAX): DECLARE_BITMAP(supported,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lynx_lane {
    pub priv: *mut lynx_priv,
    pub phy: *mut phy,
    pub powered_up: bool,
    pub init: bool,
    pub id: c_uint,
    pub mode: lynx_lane_mode,
    pub default_pccr: [u32; LANE_MODE_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lynx_info {
    pub pccr): *mut lynx_pccr,
    pub mode): *mut *mut int (get_pcvt_offset)(int lane, enum lynx_lane_mode,
    pub mode): *mut *mut bool (lane_supports_mode)(int lane, enum lynx_lane_mode,
    pub pll): *mut *mut void (pll_read_configuration)(struct lynx_pll,
    pub lane): *mut *mut void (lane_read_configuration)(struct lynx_lane,
    pub lane): *mut *mut void (cdr_lock_check)(struct lynx_lane,
    pub first_lane: c_int,
    pub num_lanes: c_int,
    pub index: c_int,
    pub quirks: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lynx_priv {
    pub base: *mut void __iomem,
    pub dev: *mut device,
    pub info: *const lynx_info,
// Serialize concurrent access to registers shared between lanes,
// like PCCn
//
    pub pcc_lock: spinlock_t,
    pub big_endian: bool,
    pub pll: [lynx_pll; LYNX_NUM_PLL],
    pub lane: *mut lynx_lane,
    pub cdr_check: delayed_work,
}

extern "C" {
    pub fn ioread32be(_arg: reg) -> return;
}
extern "C" {
    pub fn ioread32(_arg: reg) -> return;
}
extern "C" {
    pub fn iowrite32be(_arg: val, _arg: reg) -> return;
}
extern "C" {
    pub fn iowrite32(_arg: val, _arg: reg) -> return;
}

extern "C" {
    pub fn lynx_remove(pdev: *mut platform_device);
}
extern "C" {
    pub fn phy_interface_to_lane_mode(intf: phy_interface_t) -> lynx_lane_mode;
}
extern "C" {
    pub fn lynx_lane_supports_mode(lane: *mut lynx_lane, mode: lynx_lane_mode) -> bool;
}
extern "C" {
    pub fn lynx_pccr_read(lane: *mut lynx_lane, mode: lynx_lane_mode, val: *mut u32) -> c_int;
}
extern "C" {
    pub fn lynx_pccr_write(lane: *mut lynx_lane, mode: lynx_lane_mode, val: u32) -> c_int;
}
