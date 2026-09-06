//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/ixgbe/ixgbe_dcb_82599.h
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
pub const IXGBE_RTTDCS_TDPAC: c_uint = 0x00000001 /* 0 Round Robin,;
// 1 WSP - Weighted Strict Priority
//
pub const IXGBE_RTTDCS_VMPAC: c_uint = 0x00000002 /* 0 Round Robin,;
// 1 WRR - Weighted Round Robin
//
pub const IXGBE_RTTDCS_TDRM: c_uint = 0x00000010 /* Transmit Recycle Mode */;
pub const IXGBE_RTTDCS_ARBDIS: c_uint = 0x00000040 /* DCB arbiter disable */;
pub const IXGBE_RTTDCS_BDPM: c_uint = 0x00400000 /* Bypass Data Pipe - must clear! */;
pub const IXGBE_RTTDCS_BPBFSM: c_uint = 0x00800000 /* Bypass PB Free Space - must;
// clear!
//
pub const IXGBE_RTTDCS_SPEED_CHG: c_uint = 0x80000000 /* Link speed change */;
// Receive UP2TC mapping
pub const IXGBE_RTRUP2TC_UP_SHIFT: c_int = 3;
pub const IXGBE_RTRUP2TC_UP_MASK: c_int = 7;
// Transmit UP2TC mapping
pub const IXGBE_RTTUP2TC_UP_SHIFT: c_int = 3;

pub const IXGBE_RTRPT4C_GSP: c_uint = 0x40000000 /* GSP enable bit */;
pub const IXGBE_RTRPT4C_LSP: c_uint = 0x80000000 /* LSP enable bit */;
pub const IXGBE_RDRXCTL_MPBEN: c_uint = 0x00000010 /* DMA config for multiple packet;
// buffers enable
//
pub const IXGBE_RDRXCTL_MCEN: c_uint = 0x00000040 /* DMA config for multiple cores;
// (RSS) enable
//
// RTRPCS Bit Masks
pub const IXGBE_RTRPCS_RRM: c_uint = 0x00000002 /* Receive Recycle Mode enable */;
// Receive Arbitration Control: 0 Round Robin, 1 DFP
pub const IXGBE_RTRPCS_RAC: c_uint = 0x00000004;
pub const IXGBE_RTRPCS_ARBDIS: c_uint = 0x00000040 /* Arbitration disable bit */;
// RTTDT2C Bit Masks
pub const IXGBE_RTTDT2C_MCL_SHIFT: c_int = 12;
pub const IXGBE_RTTDT2C_BWG_SHIFT: c_int = 9;
pub const IXGBE_RTTDT2C_GSP: c_uint = 0x40000000;
pub const IXGBE_RTTDT2C_LSP: c_uint = 0x80000000;
pub const IXGBE_RTTPT2C_MCL_SHIFT: c_int = 12;
pub const IXGBE_RTTPT2C_BWG_SHIFT: c_int = 9;
pub const IXGBE_RTTPT2C_GSP: c_uint = 0x40000000;
pub const IXGBE_RTTPT2C_LSP: c_uint = 0x80000000;
// RTTPCS Bit Masks
pub const IXGBE_RTTPCS_TPPAC: c_uint = 0x00000020 /* 0 Round Robin,;
// 1 SP - Strict Priority
//
pub const IXGBE_RTTPCS_ARBDIS: c_uint = 0x00000040 /* Arbiter disable */;
pub const IXGBE_RTTPCS_TPRM: c_uint = 0x00000100 /* Transmit Recycle Mode enable */;
pub const IXGBE_RTTPCS_ARBD_SHIFT: c_int = 22;
pub const IXGBE_RTTPCS_ARBD_DCB: c_uint = 0x4        /* Arbitration delay in DCB mode */;
// SECTXMINIFG DCB
pub const IXGBE_SECTX_DCB: c_uint = 0x00001F00 /* DCB TX Buffer IFG */;
// DCB hardware-specific driver APIs
// DCB PFC functions
extern "C" {
    pub fn ixgbe_dcb_config_pfc_82599(hw: *mut ixgbe_hw, pfc_en: u8, prio_tc: *mut u8) -> c_int;
}
// DCB hw initialization
