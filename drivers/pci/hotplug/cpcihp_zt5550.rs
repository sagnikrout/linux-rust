//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/pci/hotplug/cpcihp_zt5550.h
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
//
// cpcihp_zt5550.h
//
// Intel/Ziatech ZT5550 CompactPCI Host Controller driver definitions
//
// Copyright 2002 SOMA Networks, Inc.
// Copyright 2001 Intel San Luis Obispo
// Copyright 2000,2001 MontaVista Software Inc.
//
// Send feedback to <scottm@somanetworks.com>
//
// Direct registers
pub const CSR_HCINDEX: c_uint = 0x00;
pub const CSR_HCDATA: c_uint = 0x04;
pub const CSR_INTSTAT: c_uint = 0x08;
pub const CSR_INTMASK: c_uint = 0x09;
pub const CSR_CNT0CMD: c_uint = 0x0C;
pub const CSR_CNT1CMD: c_uint = 0x0E;
pub const CSR_CNT0: c_uint = 0x10;
pub const CSR_CNT1: c_uint = 0x14;
// Masks for interrupt bits in CSR_INTMASK direct register
pub const CNT0_INT_MASK: c_uint = 0x01;
pub const CNT1_INT_MASK: c_uint = 0x02;
pub const ENUM_INT_MASK: c_uint = 0x04;
pub const ALL_DIRECT_INTS_MASK: c_uint = 0x07;
// Indexed registers (through CSR_INDEX, CSR_DATA)
pub const HC_INT_MASK_REG: c_uint = 0x04;
pub const HC_STATUS_REG: c_uint = 0x08;
pub const HC_CMD_REG: c_uint = 0x0C;
pub const ARB_CONFIG_GNT_REG: c_uint = 0x10;
pub const ARB_CONFIG_CFG_REG: c_uint = 0x12;
pub const ARB_CONFIG_REG: c_uint = 0x10;
pub const ISOL_CONFIG_REG: c_uint = 0x18;
pub const FAULT_STATUS_REG: c_uint = 0x20;
pub const FAULT_CONFIG_REG: c_uint = 0x24;
pub const WD_CONFIG_REG: c_uint = 0x2C;
pub const HC_DIAG_REG: c_uint = 0x30;
pub const SERIAL_COMM_REG: c_uint = 0x34;
pub const SERIAL_OUT_REG: c_uint = 0x38;
pub const SERIAL_IN_REG: c_uint = 0x3C;
// Masks for interrupt bits in HC_INT_MASK_REG indexed register
pub const SERIAL_INT_MASK: c_uint = 0x01;
pub const FAULT_INT_MASK: c_uint = 0x02;
pub const HCF_INT_MASK: c_uint = 0x04;
pub const ALL_INDEXED_INTS_MASK: c_uint = 0x07;
// Digital I/O port storing ENUM#
pub const ENUM_PORT: c_uint = 0xE1;
// Mask to get to the ENUM# bit on the bus
pub const ENUM_MASK: c_uint = 0x40;
