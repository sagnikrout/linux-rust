//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/hwtracing/coresight/coresight-catu.h
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
// Copyright (C) 2018 Arm Limited. All rights reserved.
//
// Author: Suzuki K Poulose <suzuki.poulose@arm.com>
//

// Register offset from base
pub const CATU_CONTROL: c_uint = 0x000;
pub const CATU_MODE: c_uint = 0x004;
pub const CATU_AXICTRL: c_uint = 0x008;
pub const CATU_IRQEN: c_uint = 0x00c;
pub const CATU_SLADDRLO: c_uint = 0x020;
pub const CATU_SLADDRHI: c_uint = 0x024;
pub const CATU_INADDRLO: c_uint = 0x028;
pub const CATU_INADDRHI: c_uint = 0x02c;
pub const CATU_STATUS: c_uint = 0x100;
pub const CATU_DEVARCH: c_uint = 0xfbc;
pub const CATU_CONTROL_ENABLE: c_int = 0;

pub const CATU_AXICTRL_ARCACHE_SHIFT: c_int = 4;
pub const CATU_AXICTRL_ARCACHE_MASK: c_uint = 0xf;
pub const CATU_AXICTRL_ARPROT_MASK: c_uint = 0x3;

pub const AXI3_AxCACHE_WB_READ_ALLOC: c_uint = 0x7;
//
// AXI - ARPROT bits:
// See AMBA AXI & ACE Protocol specification (ARM IHI 0022E)
// sectionA4.7 Access Permissions.
//
// Bit 0: 0 - Unprivileged access, 1 - Privileged access
// Bit 1: 0 - Secure access, 1 - Non-secure access.
// Bit 2: 0 - Data access, 1 - instruction access.
//
// CATU AXICTRL:ARPROT[2] is res0 as we always access data.
//
pub const CATU_OS_ARPROT: c_uint = 0x2;

pub const CATU_STATUS_READY: c_int = 8;
pub const CATU_STATUS_ADRERR: c_int = 0;
pub const CATU_STATUS_AXIERR: c_int = 4;
pub const CATU_IRQEN_ON: c_uint = 0x1;
pub const CATU_IRQEN_OFF: c_uint = 0x0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct catu_drvdata {
    pub pclk: *mut clk,
    pub atclk: *mut clk,
    pub base: *mut void __iomem,
    pub csdev: *mut coresight_device,
    pub irq: c_int,
    pub spinlock: raw_spinlock_t,
}

