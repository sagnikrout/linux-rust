//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/tpm_ptp.h
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
// Following copyright information was take from the original file
// <drivers/char/tpm/tpm_tis_core.h> where the definitions were moved
// from:
//
// Copyright (C) 2005, 2006 IBM Corporation
// Copyright (C) 2014, 2015 Intel Corporation
//
// Authors:
// Leendert van Doorn <leendert@watson.ibm.com>
// Kylene Hall <kjhall@us.ibm.com>
//
// Maintained by: <tpmdd-devel@lists.sourceforge.net>
//
// Device driver for TCG/TCPA TPM (trusted platform module).
// Specifications at www.trustedcomputinggroup.org
//
// This device driver implements the TPM interface as defined in
// the TCG TPM Interface Spec version 1.2, revision 1.0.
//
// TCG PC Client Platform TPM Profile (PTP) Specification
// https://trustedcomputinggroup.org/resource/pc-client-platform-tpm-profile-ptp-specification
//
// TIS/FIFO macros and definitions
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tis_access {
    TPM_ACCESS_VALID		= 0x80,
    TPM_ACCESS_ACTIVE_LOCALITY	= 0x20,	/* (R) */
    TPM_ACCESS_RELINQUISH_LOCALITY	= 0x20, /* (W) */
    TPM_ACCESS_REQUEST_PENDING	= 0x04,	/* (W) */
    TPM_ACCESS_REQUEST_USE		= 0x02,	/* (W) */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tis_status {
    TPM_STS_VALID		= 0x80, /* (R) */
    TPM_STS_COMMAND_READY	= 0x40, /* (R) */
    TPM_STS_DATA_AVAIL	= 0x10, /* (R) */
    TPM_STS_DATA_EXPECT	= 0x08, /* (R) */
    TPM_STS_GO		= 0x20, /* (W) */
    TPM_STS_RESPONSE_RETRY	= 0x02, /* (R) */
    TPM_STS_READ_ZERO	= 0x23, /* bits that must be zero on read */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tis_int_flags {
    TPM_GLOBAL_INT_ENABLE		= 0x80000000,
    TPM_INTF_BURST_COUNT_STATIC	= 0x100,
    TPM_INTF_CMD_READY_INT		= 0x080,
    TPM_INTF_INT_EDGE_FALLING	= 0x040,
    TPM_INTF_INT_EDGE_RISING	= 0x020,
    TPM_INTF_INT_LEVEL_LOW		= 0x010,
    TPM_INTF_INT_LEVEL_HIGH		= 0x008,
    TPM_INTF_LOCALITY_CHANGE_INT	= 0x004,
    TPM_INTF_STS_VALID_INT		= 0x002,
    TPM_INTF_DATA_AVAIL_INT		= 0x001,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tis_defaults {
    TIS_MEM_LEN		= 0x5000,
    TIS_SHORT_TIMEOUT	= 750,   /* ms */
    TIS_LONG_TIMEOUT	= 4000,  /* 4 secs */
    TIS_TIMEOUT_MIN_ATML	= 14700, /* usecs */
    TIS_TIMEOUT_MAX_ATML	= 15000, /* usecs */
}

pub const TIS_MEM_X86_LPC_BASE: c_uint = 0xFED40000;
pub const INTEL_LEGACY_BLK_BASE_ADDR: c_uint = 0xFED08000;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tis_x86_defaults {
    TIS_MEM_X86_LEN			= 0x5000,
    ILB_REMAP_SIZE			= 0x100,
    LPC_CNTRL_OFFSET		= 0x84,
    LPC_CLKRUN_EN			= (1 << 2),
}

//
// Some timeout values are needed before it is known whether the chip is
// TPM 1.0 or TPM 2.0.
//

// TPM HW Interface and Capabilities
pub const TPM_TIS_INTF_ACTIVE: c_uint = 0x00;
pub const TPM_CRB_INTF_ACTIVE: c_uint = 0x01;

pub const TPM_TIS_INTF_12: c_uint = 0x00;
pub const TPM_TIS_INTF_13: c_uint = 0x02;
pub const TPM2_TIS_INTF_13: c_uint = 0x03;

