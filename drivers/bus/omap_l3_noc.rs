//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/bus/omap_l3_noc.h
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
// OMAP L3 Interconnect  error handling driver header
//
// Copyright (C) 2011-2015 Texas Instruments Incorporated - http://www.ti.com
// Santosh Shilimkar <santosh.shilimkar@ti.com>
// sricharan <r.sricharan@ti.com>
//
pub const MAX_L3_MODULES: c_int = 3;
pub const MAX_CLKDM_TARGETS: c_int = 31;

pub const CUSTOM_ERROR: c_uint = 0x2;
pub const STANDARD_ERROR: c_uint = 0x0;
pub const INBAND_ERROR: c_uint = 0x0;
pub const L3_APPLICATION_ERROR: c_uint = 0x0;
pub const L3_DEBUG_ERROR: c_uint = 0x1;
// L3 TARG register offsets
pub const L3_TARG_STDERRLOG_MAIN: c_uint = 0x48;
pub const L3_TARG_STDERRLOG_HDR: c_uint = 0x4c;
pub const L3_TARG_STDERRLOG_MSTADDR: c_uint = 0x50;
pub const L3_TARG_STDERRLOG_INFO: c_uint = 0x58;
pub const L3_TARG_STDERRLOG_SLVOFSLSB: c_uint = 0x5c;
pub const L3_TARG_STDERRLOG_CINFO_INFO: c_uint = 0x64;
pub const L3_TARG_STDERRLOG_CINFO_MSTADDR: c_uint = 0x68;
pub const L3_TARG_STDERRLOG_CINFO_OPCODE: c_uint = 0x6c;
pub const L3_FLAGMUX_REGERR0: c_uint = 0xc;
pub const L3_FLAGMUX_MASK0: c_uint = 0x8;

// 0 0 0 */ "Idle",
// 0 0 1 */ "Write",
// 0 1 0 */ "Read",
// 0 1 1 */ "ReadEx",
// 1 0 0 */ "Read Link",
// 1 0 1 */ "Write Non-Posted",
// 1 1 0 */ "Write Conditional",
// 1 1 1 */ "Write Broadcast",
//
// struct l3_masters_data - L3 Master information
// @id:		ID of the L3 Master
// @name:	master name
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct l3_masters_data {
    pub id: u32,
    pub name: *mut c_char,
}

//
// struct l3_target_data - L3 Target information
// @offset:	Offset from base for L3 Target
// @name:	Target name
//
// Target information is organized indexed by bit field definitions.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct l3_target_data {
    pub offset: u32,
    pub name: *mut c_char,
}

//
// struct l3_flagmux_data - Flag Mux information
// @offset:	offset from base for flagmux register
// @l3_targ:	array indexed by flagmux index (bit offset) pointing to the
// target data. unsupported ones are marked with
// L3_TARGET_NOT_SUPPORTED
// @num_targ_data: number of entries in target data
// @mask_app_bits: ignore these from raw application irq status
// @mask_dbg_bits: ignore these from raw debug irq status
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct l3_flagmux_data {
    pub offset: u32,
    pub l3_targ: *mut l3_target_data,
    pub num_targ_data: u8,
    pub mask_app_bits: u32,
    pub mask_dbg_bits: u32,
}

//
// struct omap_l3 - Description of data relevant for L3 bus.
// @dev:	device representing the bus (populated runtime)
// @l3_base:	base addresses of modules (populated runtime if 0)
// if set to L3_BASE_IS_SUBMODULE, then uses previous
// module index as the base address
// @l3_flag_mux: array containing flag mux data per module
// offset from corresponding module base indexed per
// module.
// @num_modules: number of clock domains / modules.
// @l3_masters:	array pointing to master data containing name and register
// offset for the master.
// @num_master: number of masters
// @mst_addr_mask: Mask representing MSTADDR information of NTTP packet
// @debug_irq:	irq number of the debug interrupt (populated runtime)
// @app_irq:	irq number of the application interrupt (populated runtime)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_l3 {
    pub dev: *mut device,
    pub l3_base: [*mut void __iomem; MAX_L3_MODULES],
    pub l3_flagmux: *mut l3_flagmux_data,
    pub num_modules: c_int,
    pub l3_masters: *mut l3_masters_data,
    pub num_masters: c_int,
    pub mst_addr_mask: u32,
    pub debug_irq: c_int,
    pub app_irq: c_int,
}

// The 6 MSBs of register field used to distinguish initiator
// OMAP5 data
// The 6 MSBs of register field used to distinguish initiator
// DRA7 data
// The 6 MSBs of register field used to distinguish initiator
// AM4372 data
// All 6 bits of register field used to distinguish initiator
