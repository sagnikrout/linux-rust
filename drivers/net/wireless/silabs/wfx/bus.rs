//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/silabs/wfx/bus.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Common bus abstraction layer.
//
// Copyright (c) 2017-2020, Silicon Laboratories, Inc.
// Copyright (c) 2010, ST-Ericsson
//

pub const WFX_REG_CONFIG: c_uint = 0x0;
pub const WFX_REG_CONTROL: c_uint = 0x1;
pub const WFX_REG_IN_OUT_QUEUE: c_uint = 0x2;
pub const WFX_REG_AHB_DPORT: c_uint = 0x3;
pub const WFX_REG_BASE_ADDR: c_uint = 0x4;
pub const WFX_REG_SRAM_DPORT: c_uint = 0x5;
pub const WFX_REG_SET_GEN_R_W: c_uint = 0x6;
pub const WFX_REG_FRAME_OUT: c_uint = 0x7;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wfx_hwbus_ops {
    pub count): *mut *mut *mut *mut int (copy_from_io)(void bus_priv, unsigned int addr, void dst, size_t,
    pub count): *const *const *const *const int (copy_to_io)(void bus_priv, unsigned int addr, void src, size_t,
    pub bus_priv): *mut *mut int (irq_subscribe)(void,
    pub bus_priv): *mut *mut int (irq_unsubscribe)(void,
    pub bus_priv): *mut *mut void (lock)(void,
    pub bus_priv): *mut *mut void (unlock)(void,
    pub size): *mut *mut *mut size_t (align_size)(void bus_priv, size_t,
    pub enabled): *mut *mut *mut void (set_wakeup)(void priv, bool,
}
