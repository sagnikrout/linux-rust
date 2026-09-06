//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/ixgbe/ixgbe_dcb_82598.h
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
// Copyright(c) 1999 - 2018 Intel Corporation.
// DCB register definitions
pub const IXGBE_DPMCS_MTSOS_SHIFT: c_int = 16;
pub const IXGBE_DPMCS_TDPAC: c_uint = 0x00000001 /* 0 Round Robin, 1 DFP - Deficit Fixed Priority */;
pub const IXGBE_DPMCS_TRM: c_uint = 0x00000010 /* Transmit Recycle Mode */;
pub const IXGBE_DPMCS_ARBDIS: c_uint = 0x00000040 /* DCB arbiter disable */;
pub const IXGBE_DPMCS_TSOEF: c_uint = 0x00080000 /* TSO Expand Factor: 0=x4, 1=x2 */;
pub const IXGBE_RUPPBMR_MQA: c_uint = 0x80000000 /* Enable UP to queue mapping */;

pub const IXGBE_RT2CR_LSP: c_uint = 0x80000000 /* LSP enable bit */;
pub const IXGBE_RDRXCTL_MPBEN: c_uint = 0x00000010 /* DMA config for multiple packet buffers enable */;
pub const IXGBE_RDRXCTL_MCEN: c_uint = 0x00000040 /* DMA config for multiple cores (RSS) enable */;
pub const IXGBE_TDTQ2TCCR_MCL_SHIFT: c_int = 12;
pub const IXGBE_TDTQ2TCCR_BWG_SHIFT: c_int = 9;
pub const IXGBE_TDTQ2TCCR_GSP: c_uint = 0x40000000;
pub const IXGBE_TDTQ2TCCR_LSP: c_uint = 0x80000000;
pub const IXGBE_TDPT2TCCR_MCL_SHIFT: c_int = 12;
pub const IXGBE_TDPT2TCCR_BWG_SHIFT: c_int = 9;
pub const IXGBE_TDPT2TCCR_GSP: c_uint = 0x40000000;
pub const IXGBE_TDPT2TCCR_LSP: c_uint = 0x80000000;
pub const IXGBE_PDPMCS_TPPAC: c_uint = 0x00000020 /* 0 Round Robin, 1 for DFP - Deficit Fixed Priority */;
pub const IXGBE_PDPMCS_ARBDIS: c_uint = 0x00000040 /* Arbiter disable */;
pub const IXGBE_PDPMCS_TRM: c_uint = 0x00000100 /* Transmit Recycle Mode enable */;
pub const IXGBE_DTXCTL_ENDBUBD: c_uint = 0x00000004 /* Enable DBU buffer division */;
pub const IXGBE_TXPBSIZE_40KB: c_uint = 0x0000A000 /* 40KB Packet Buffer */;
pub const IXGBE_RXPBSIZE_48KB: c_uint = 0x0000C000 /* 48KB Packet Buffer */;
pub const IXGBE_RXPBSIZE_64KB: c_uint = 0x00010000 /* 64KB Packet Buffer */;
pub const IXGBE_RXPBSIZE_80KB: c_uint = 0x00014000 /* 80KB Packet Buffer */;
pub const IXGBE_RDRXCTL_RDMTS_1_2: c_uint = 0x00000000;
// DCB hardware-specific driver APIs
// DCB PFC functions
extern "C" {
    pub fn ixgbe_dcb_config_pfc_82598(: *mut ixgbe_hw, pfc_en: u8) -> c_int;
}
// DCB hw initialization
