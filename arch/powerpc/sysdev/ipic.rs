//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/sysdev/ipic.h
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
// IPIC private definitions and structure.
//
// Maintainer: Kumar Gala <galak@kernel.crashing.org>
//
// Copyright 2005 Freescale Semiconductor, Inc
//

pub const NR_IPIC_INTS: c_int = 128;
// External IRQS
pub const IPIC_IRQ_EXT0: c_int = 48;
pub const IPIC_IRQ_EXT1: c_int = 17;
pub const IPIC_IRQ_EXT7: c_int = 23;
// Default Priority Registers
pub const IPIC_PRIORITY_DEFAULT: c_uint = 0x05309770;
// System Global Interrupt Configuration Register
pub const SICFR_IPSA: c_uint = 0x00010000;
pub const SICFR_IPSB: c_uint = 0x00020000;
pub const SICFR_IPSC: c_uint = 0x00040000;
pub const SICFR_IPSD: c_uint = 0x00080000;
pub const SICFR_MPSA: c_uint = 0x00200000;
pub const SICFR_MPSB: c_uint = 0x00400000;
// System External Interrupt Mask Register
pub const SEMSR_SIRQ0: c_uint = 0x00008000;
// System Error Control Register
pub const SERCR_MCPR: c_uint = 0x00000001;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipic {
    pub regs: *mut volatile u32 __iomem,
// The remapper for this IPIC
    pub irqhost: *mut irq_domain,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipic_info {
    pub irq: *mut *mut u8 ack; / pending register offset from base if the,
    pub /: *mut *mut u8 mask; / mask register offset from base,
    pub /: *mut *mut u8 prio; / priority register offset from base,
    pub /: *mut *mut u8 force; / force register offset from base,
    pub doc): *mut *mut u8 bit; / register bit position (as per,
    pub /: *mut *mut u8 prio_mask; / priority mask value,
}
