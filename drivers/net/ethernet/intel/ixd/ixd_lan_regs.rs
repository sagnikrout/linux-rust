//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/ixd/ixd_lan_regs.h
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
// Copyright (C) 2025 Intel Corporation
// Control Plane Function PCI ID
pub const IXD_DEV_ID_CPF: c_uint = 0x1efe;
// Control Queue (Mailbox)
pub const PF_FW_MBX_REG_LEN: c_int = 4096;
pub const PF_FW_MBX: c_uint = 0x08400000;

pub const PF_FW_ARQLEN_ARQENABLE_S: c_int = 31;

pub const PF_FW_ATQLEN_ATQENABLE_S: c_int = 31;

// Reset registers
pub const PFGEN_RTRIG_REG_LEN: c_int = 2048;
pub const PFGEN_RTRIG: c_uint = 0x08407000	/* Device resets */;
pub const PFGEN_RSTAT: c_uint = 0x08407008	/* PFR status */;

pub const PFGEN_CTRL: c_uint = 0x0840700C	/* PFR trigger */;

//
// struct ixd_bar_region - BAR region description
// @offset: BAR region offset
// @size: BAR region size
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixd_bar_region {
    pub offset: resource_size_t,
    pub size: resource_size_t,
}

//
// struct ixd_reset_reg - structure for reset registers
// @rstat: offset of status in register
// @rstat_m: status mask
// @rstat_ok_v: value that indicates PFR completed status
// @rtrigger: offset of reset trigger in register
// @rtrigger_m: reset trigger mask
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixd_reset_reg {
    pub rstat: u32,
    pub rstat_m: u32,
    pub rstat_ok_v: u32,
    pub rtrigger: u32,
    pub rtrigger_m: u32,
}
