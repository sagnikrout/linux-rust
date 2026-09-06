//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/i3c/master/dw-i3c-master.h
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
// Copyright (c) 2023 Code Construct
//
// Author: Jeremy Kerr <jk@codeconstruct.com.au>
//

pub const DW_I3C_MAX_DEVS: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_i3c_master_caps {
    pub cmdfifodepth: u8,
    pub datafifodepth: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_i3c_dat_entry {
    pub addr: u8,
    pub is_i2c_addr: bool,
    pub ibi_dev: *mut i3c_dev_desc,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_i3c_master {
    pub base: i3c_master_controller,
    pub dev: *mut device,
    pub maxdevs: u16,
    pub datstartaddr: u16,
    pub free_pos: u32,
    pub list: list_head,
    pub cur: *mut dw_i3c_xfer,
    pub lock: spinlock_t,
    pub xferqueue: },
    pub caps: dw_i3c_master_caps,
    pub regs: *mut void __iomem,
    pub core_rst: *mut reset_control,
    pub core_clk: *mut clk,
    pub pclk: *mut clk,
    pub version: [c_char; 5],
    pub type: [c_char; 5],
    pub sir_rej_mask: u32,
    pub i2c_slv_prsnt: bool,
    pub dev_addr: u32,
    pub i3c_pp_timing: u32,
    pub i3c_od_timing: u32,
    pub i3c_od_timing_normal: u32,
    pub ext_lcnt_timing: u32,
    pub bus_free_timing: u32,
    pub i2c_fm_timing: u32,
    pub i2c_fmp_timing: u32,
    pub quirks: u32,
    pub has_ibi_data: bool,
//
// Per-device hardware data, used to manage the device address table
// (DAT)
//
// Locking: the devs array may be referenced in IRQ context while
// processing an IBI. However, IBIs (for a specific device, which
// implies a specific DAT entry) can only happen while interrupts are
// requested for that device, which is serialised against other
// insertions/removals from the array by the global i3c infrastructure.
// So, devs_lock protects against concurrent updates to devs->ibi_dev
// between request_ibi/free_ibi and the IBI irq event.
//
    pub devs: [dw_i3c_dat_entry; DW_I3C_MAX_DEVS],
    pub devs_lock: spinlock_t,
// platform-specific data
    pub platform_ops: *const dw_i3c_platform_ops,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_i3c_platform_ops {
//
// Called on early bus init: the i3c has been set up, but before any
// transactions have taken place. Platform implementations may use to
// perform actual device enabling with the i3c core ready.
//
    pub i3c): *mut *mut int (init)(struct dw_i3c_master,
//
// Initialise a DAT entry to enable/disable IBIs. Allows the platform
// to perform any device workarounds on the DAT entry before
// inserting into the hardware table.
//
// Called with the DAT lock held; must not sleep.
//
    pub reg): *mut *mut i3c_dev_desc dev, bool enable, u32,
}

extern "C" {
    pub fn dw_i3c_common_remove(master: *mut dw_i3c_master);
}
