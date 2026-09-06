//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/char/tpm/tpm.h
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
// Copyright (C) 2004 IBM Corporation
// Copyright (C) 2015 Intel Corporation
//
// Authors:
// Leendert van Doorn <leendert@watson.ibm.com>
// Dave Safford <safford@watson.ibm.com>
// Reiner Sailer <sailer@watson.ibm.com>
// Kylene Hall <kjhall@us.ibm.com>
//
// Maintained by: <tpmdd-devel@lists.sourceforge.net>
//
// Device driver for TCG/TCPA TPM (trusted platform module).
// Specifications at www.trustedcomputinggroup.org
//

pub const TPM_NUM_DEVICES: c_int = 65536;
pub const TPM_RETRY: c_int = 50;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tpm_timeout {
    TPM_TIMEOUT = 5,	/* msecs */
    TPM_TIMEOUT_RETRY = 100, /* msecs */
    TPM_TIMEOUT_RANGE_US = 300,	/* usecs */
    TPM_TIMEOUT_POLL = 1,	/* msecs */
    TPM_TIMEOUT_USECS_MIN = 100,      /* usecs */
    TPM_TIMEOUT_USECS_MAX = 500      /* usecs */
}

// TPM addresses
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tpm_addr {
    TPM_SUPERIO_ADDR = 0x2E,
    TPM_ADDR = 0x4E,
}

extern "C" {
    pub fn tpm_transmit(chip: *mut tpm_chip, buf: *mut u8, bufsiz: usize) -> isize;
}
extern "C" {
    pub fn tpm_get_timeouts(: *mut tpm_chip) -> c_int;
}
extern "C" {
    pub fn tpm_auto_startup(chip: *mut tpm_chip) -> c_int;
}
extern "C" {
    pub fn tpm1_pm_suspend(chip: *mut tpm_chip, tpm_suspend_pcr: u32) -> c_int;
}
extern "C" {
    pub fn tpm1_auto_startup(chip: *mut tpm_chip) -> c_int;
}
extern "C" {
    pub fn tpm1_do_selftest(chip: *mut tpm_chip) -> c_int;
}
extern "C" {
    pub fn tpm1_get_timeouts(chip: *mut tpm_chip) -> c_int;
}
extern "C" {
    pub fn tpm1_calc_ordinal_duration(chip: *mut tpm_chip, ordinal: u32) -> c_ulong;
}
extern "C" {
    pub fn tpm1_pcr_read(chip: *mut tpm_chip, pcr_idx: u32, res_buf: *mut u8) -> c_int;
}
extern "C" {
    pub fn tpm1_get_random(chip: *mut tpm_chip, out: *mut u8, max: usize) -> c_int;
}
extern "C" {
    pub fn tpm1_get_pcr_allocation(chip: *mut tpm_chip) -> c_int;
}
extern "C" {
    pub fn tpm_calc_ordinal_duration(chip: *mut tpm_chip, ordinal: u32) -> c_ulong;
}
extern "C" {
    pub fn tpm_pm_suspend(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn tpm_pm_resume(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn tpm_class_shutdown(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn tpm_chip_bootstrap(chip: *mut tpm_chip) -> c_int;
}
extern "C" {
    pub fn tpm_chip_start(chip: *mut tpm_chip) -> c_int;
}
extern "C" {
    pub fn tpm_chip_stop(chip: *mut tpm_chip);
}
extern "C" {
    pub fn tpm_chip_register(chip: *mut tpm_chip) -> c_int;
}
extern "C" {
    pub fn tpm_chip_unregister(chip: *mut tpm_chip);
}
extern "C" {
    pub fn tpm_sysfs_add_device(chip: *mut tpm_chip);
}

extern "C" {
    pub fn tpm_add_ppi(chip: *mut tpm_chip);
}

extern "C" {
    pub fn tpm2_get_timeouts(chip: *mut tpm_chip) -> c_int;
}
extern "C" {
    pub fn tpm2_get_random(chip: *mut tpm_chip, dest: *mut u8, max: usize) -> c_int;
}
extern "C" {
    pub fn tpm2_get_pcr_allocation(chip: *mut tpm_chip) -> isize;
}
extern "C" {
    pub fn tpm2_auto_startup(chip: *mut tpm_chip) -> c_int;
}
extern "C" {
    pub fn tpm2_shutdown(chip: *mut tpm_chip, shutdown_type: u16);
}
extern "C" {
    pub fn tpm2_calc_ordinal_duration(ordinal: u32) -> c_ulong;
}
extern "C" {
    pub fn tpm2_probe(chip: *mut tpm_chip) -> c_int;
}
extern "C" {
    pub fn tpm2_get_cc_attrs_tbl(chip: *mut tpm_chip) -> c_int;
}
extern "C" {
    pub fn tpm2_find_cc(chip: *mut tpm_chip, cc: u32) -> c_int;
}
extern "C" {
    pub fn tpm2_init_space(space: *mut tpm_space, buf_size: c_uint) -> c_int;
}
extern "C" {
    pub fn tpm2_del_space(chip: *mut tpm_chip, space: *mut tpm_space);
}
extern "C" {
    pub fn tpm2_flush_space(chip: *mut tpm_chip);
}
extern "C" {
    pub fn tpm_devs_add(chip: *mut tpm_chip) -> c_int;
}
extern "C" {
    pub fn tpm_devs_remove(chip: *mut tpm_chip);
}
extern "C" {
    pub fn tpm_bios_log_setup(chip: *mut tpm_chip);
}
extern "C" {
    pub fn tpm_bios_log_teardown(chip: *mut tpm_chip);
}
extern "C" {
    pub fn tpm_dev_common_init() -> c_int;
}
extern "C" {
    pub fn tpm_dev_common_exit();
}

extern "C" {
    pub fn tpm2_sessions_init(chip: *mut tpm_chip) -> c_int;
}

