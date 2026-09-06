//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/sysdev/fsl_msi.h
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
// Copyright (C) 2007-2008 Freescale Semiconductor, Inc. All rights reserved.
//
// Author: Tony Li <tony.li@freescale.com>
// Jason Jin <Jason.jin@freescale.com>
//

pub const IRQS_PER_MSI_REG: c_int = 32;

pub const FSL_PIC_IP_MASK: c_uint = 0x0000000F;
pub const FSL_PIC_IP_MPIC: c_uint = 0x00000001;
pub const FSL_PIC_IP_IPIC: c_uint = 0x00000002;
pub const FSL_PIC_IP_VMPIC: c_uint = 0x00000003;
pub const MSI_HW_ERRATA_ENDIAN: c_uint = 0x00000010;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_msi {
    pub irqhost: *mut irq_domain,
    pub cascade_irq: c_ulong,
    pub /: *mut *mut u32 msiir_offset; / Offset of MSIIR, relative to start of CCSR,
    pub /: *mut *mut u32 ibs_shift; / Shift of interrupt bit select,
    pub /: *mut *mut u32 srs_shift; / Shift of the shared interrupt register select,
    pub msi_regs: *mut void __iomem,
    pub feature: u32,
    pub cascade_array: [*mut fsl_msi_cascade_data; NR_MSI_REG_MAX],
    pub bitmap: msi_bitmap,
    pub /: *mut *mut list_head list; / support multiple MSI banks,
    pub phandle: phandle,
}
