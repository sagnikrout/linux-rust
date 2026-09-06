//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/ti/omap-mcpdm.h
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
// omap-mcpdm.h
//
// Copyright (C) 2009 - 2011 Texas Instruments
//
// Contact: Misael Lopez Cruz <misael.lopez@ti.com>
//
pub const MCPDM_REG_REVISION: c_uint = 0x00;
pub const MCPDM_REG_SYSCONFIG: c_uint = 0x10;
pub const MCPDM_REG_IRQSTATUS_RAW: c_uint = 0x24;
pub const MCPDM_REG_IRQSTATUS: c_uint = 0x28;
pub const MCPDM_REG_IRQENABLE_SET: c_uint = 0x2C;
pub const MCPDM_REG_IRQENABLE_CLR: c_uint = 0x30;
pub const MCPDM_REG_IRQWAKE_EN: c_uint = 0x34;
pub const MCPDM_REG_DMAENABLE_SET: c_uint = 0x38;
pub const MCPDM_REG_DMAENABLE_CLR: c_uint = 0x3C;
pub const MCPDM_REG_DMAWAKEEN: c_uint = 0x40;
pub const MCPDM_REG_CTRL: c_uint = 0x44;
pub const MCPDM_REG_DN_DATA: c_uint = 0x48;
pub const MCPDM_REG_UP_DATA: c_uint = 0x4C;
pub const MCPDM_REG_FIFO_CTRL_DN: c_uint = 0x50;
pub const MCPDM_REG_FIFO_CTRL_UP: c_uint = 0x54;
pub const MCPDM_REG_DN_OFFSET: c_uint = 0x58;
//
// MCPDM_IRQ bit fields
// IRQSTATUS_RAW, IRQSTATUS, IRQENABLE_SET, IRQENABLE_CLR
//

pub const MCPDM_DOWNLINK_IRQ_MASK: c_uint = 0x00F;
pub const MCPDM_UPLINK_IRQ_MASK: c_uint = 0xF00;
//
// MCPDM_DMAENABLE bit fields
//

//
// MCPDM_CTRL bit fields
//

pub const MCPDM_PDM_UP_MASK: c_uint = 0x7;

//
// MCPDM_FIFO_CTRL bit fields
//
pub const MCPDM_UP_THRES_MAX: c_uint = 0xF;
pub const MCPDM_DN_THRES_MAX: c_uint = 0xF;
//
// MCPDM_DN_OFFSET bit fields
//

