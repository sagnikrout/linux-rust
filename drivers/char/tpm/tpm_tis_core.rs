//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/char/tpm/tpm_tis_core.h
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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tpm_tis_flags {
    TPM_TIS_ITPM_WORKAROUND		= 0,
    TPM_TIS_INVALID_STATUS		= 1,
    TPM_TIS_DEFAULT_CANCELLATION	= 2,
    TPM_TIS_IRQ_TESTED		= 3,
    TPM_TIS_STATUS_VALID_RETRY	= 4,
    TPM_TIS_SETTLE_AFTER_RELINQUISH	= 5,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tpm_tis_data {
    pub chip: *mut tpm_chip,
    pub did_vid: u32,
    pub locality_count_mutex: mutex,
    pub locality_count: c_uint,
    pub locality: c_int,
    pub irq: c_int,
    pub free_irq_work: work_struct,
    pub last_unhandled_irq: c_ulong,
    pub unhandled_irqs: c_uint,
    pub int_mask: c_uint,
    pub flags: c_ulong,
    pub ilb_base_addr: *mut void __iomem,
    pub clkrun_enabled: u16,
    pub int_queue: wait_queue_head_t,
    pub read_queue: wait_queue_head_t,
    pub phy_ops: *const tpm_tis_phy_ops,
    pub rng_quality: c_ushort,
    pub /: *mut *mut unsigned int timeout_min; / usecs,
    pub /: *mut *mut unsigned int timeout_max; / usecs,
}

//
// IO modes to indicate how many bytes should be read/written at once in the
// tpm_tis_phy_ops read_bytes/write_bytes calls. Use TPM_TIS_PHYS_8 to
// receive/transmit byte-wise, TPM_TIS_PHYS_16 for two bytes etc.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tpm_tis_io_mode {
    TPM_TIS_PHYS_8,
    TPM_TIS_PHYS_16,
    TPM_TIS_PHYS_32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tpm_tis_phy_ops {
// data is passed in little endian
    pub mode): *mut *mut u8 result, enum tpm_tis_io_mode,
    pub mode): *const *const u8 value, enum tpm_tis_io_mode,
    pub value): *const u8,
}

// result = le16_to_cpu(result_le);
// result = le32_to_cpu(result_le);

extern "C" {
    pub fn tpm_tis_remove(chip: *mut tpm_chip);
}

extern "C" {
    pub fn tpm_tis_resume(dev: *mut device) -> c_int;
}

