//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/ipic.h
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
// IPIC external definitions and structure.
//
// Maintainer: Kumar Gala <galak@kernel.crashing.org>
//
// Copyright 2005 Freescale Semiconductor, Inc
//

// Flags when we init the IPIC
pub const IPIC_SPREADMODE_GRP_A: c_uint = 0x00000001;
pub const IPIC_SPREADMODE_GRP_B: c_uint = 0x00000002;
pub const IPIC_SPREADMODE_GRP_C: c_uint = 0x00000004;
pub const IPIC_SPREADMODE_GRP_D: c_uint = 0x00000008;
pub const IPIC_SPREADMODE_MIX_A: c_uint = 0x00000010;
pub const IPIC_SPREADMODE_MIX_B: c_uint = 0x00000020;
pub const IPIC_DISABLE_MCP_OUT: c_uint = 0x00000040;
pub const IPIC_IRQ0_MCP: c_uint = 0x00000080;
// IPIC registers offsets
pub const IPIC_SICFR: c_uint = 0x00	/* System Global Interrupt Configuration Register */;
pub const IPIC_SIVCR: c_uint = 0x04	/* System Global Interrupt Vector Register */;
pub const IPIC_SIPNR_H: c_uint = 0x08	/* System Internal Interrupt Pending Register (HIGH) */;
pub const IPIC_SIPNR_L: c_uint = 0x0C	/* System Internal Interrupt Pending Register (LOW) */;
pub const IPIC_SIPRR_A: c_uint = 0x10	/* System Internal Interrupt group A Priority Register */;
pub const IPIC_SIPRR_B: c_uint = 0x14	/* System Internal Interrupt group B Priority Register */;
pub const IPIC_SIPRR_C: c_uint = 0x18	/* System Internal Interrupt group C Priority Register */;
pub const IPIC_SIPRR_D: c_uint = 0x1C	/* System Internal Interrupt group D Priority Register */;
pub const IPIC_SIMSR_H: c_uint = 0x20	/* System Internal Interrupt Mask Register (HIGH) */;
pub const IPIC_SIMSR_L: c_uint = 0x24	/* System Internal Interrupt Mask Register (LOW) */;
pub const IPIC_SICNR: c_uint = 0x28	/* System Internal Interrupt Control Register */;
pub const IPIC_SEPNR: c_uint = 0x2C	/* System External Interrupt Pending Register */;
pub const IPIC_SMPRR_A: c_uint = 0x30	/* System Mixed Interrupt group A Priority Register */;
pub const IPIC_SMPRR_B: c_uint = 0x34	/* System Mixed Interrupt group B Priority Register */;
pub const IPIC_SEMSR: c_uint = 0x38	/* System External Interrupt Mask Register */;
pub const IPIC_SECNR: c_uint = 0x3C	/* System External Interrupt Control Register */;
pub const IPIC_SERSR: c_uint = 0x40	/* System Error Status Register */;
pub const IPIC_SERMR: c_uint = 0x44	/* System Error Mask Register */;
pub const IPIC_SERCR: c_uint = 0x48	/* System Error Control Register */;
pub const IPIC_SIFCR_H: c_uint = 0x50	/* System Internal Interrupt Force Register (HIGH) */;
pub const IPIC_SIFCR_L: c_uint = 0x54	/* System Internal Interrupt Force Register (LOW) */;
pub const IPIC_SEFCR: c_uint = 0x58	/* System External Interrupt Force Register */;
pub const IPIC_SERFR: c_uint = 0x5C	/* System Error Force Register */;
pub const IPIC_SCVCR: c_uint = 0x60	/* System Critical Interrupt Vector Register */;
pub const IPIC_SMVCR: c_uint = 0x64	/* System Management Interrupt Vector Register */;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ipic_prio_grp {
    IPIC_INT_GRP_A = IPIC_SIPRR_A,
    IPIC_INT_GRP_D = IPIC_SIPRR_D,
    IPIC_MIX_GRP_A = IPIC_SMPRR_A,
    IPIC_MIX_GRP_B = IPIC_SMPRR_B,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ipic_mcp_irq {
    IPIC_MCP_IRQ0 = 0,
    IPIC_MCP_WDT  = 1,
    IPIC_MCP_SBA  = 2,
    IPIC_MCP_PCI1 = 5,
    IPIC_MCP_PCI2 = 6,
    IPIC_MCP_MU   = 7,
}

extern "C" {
    pub fn ipic_set_default_priority() -> void __init;
}
extern "C" {
    pub fn ipic_get_mcp_status() -> u32;
}
extern "C" {
    pub fn ipic_clear_mcp_status(mask: u32);
}
extern "C" {
    pub fn ipic_init(node: *mut device_node, flags: c_uint) -> *mut ipic;
}
extern "C" {
    pub fn ipic_get_irq() -> c_uint;
}

