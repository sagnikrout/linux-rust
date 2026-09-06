//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/tpm.h
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
// Copyright (C) 2004,2007,2008 IBM Corporation
//
// Authors:
// Leendert van Doorn <leendert@watson.ibm.com>
// Dave Safford <safford@watson.ibm.com>
// Reiner Sailer <sailer@watson.ibm.com>
// Kylene Hall <kjhall@us.ibm.com>
// Debora Velarde <dvelarde@us.ibm.com>
//
// Maintained by: <tpmdd_devel@lists.sourceforge.net>
//
// Device driver for TCG/TCPA TPM (trusted platform module).
// Specifications at www.trustedcomputinggroup.org
//

// opaque structure, holds auth session parameters like the session key
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum TPM_OPS_FLAGS {
    TPM_OPS_AUTO_STARTUP = BIT(0),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tpm_class_ops {
    pub flags: c_uint,
    pub req_complete_mask: u8,
    pub req_complete_val: u8,
    pub status): *mut *mut *mut bool (req_canceled)(struct tpm_chip chip, u8,
    pub len): *mut *mut *mut *mut int (recv) (struct tpm_chip chip, u8 buf, size_t,
    pub cmd_len): usize,
    pub chip): *mut *mut void (cancel) (struct tpm_chip,
    pub chip): *mut *mut u8 (status) (struct tpm_chip,
    pub timeout_cap): *mut c_ulong,
    pub duration_cap): *mut c_ulong,
    pub chip): *mut *mut int (go_idle)(struct tpm_chip,
    pub chip): *mut *mut int (cmd_ready)(struct tpm_chip,
    pub loc): *mut *mut *mut int (request_locality)(struct tpm_chip chip, int,
    pub loc): *mut *mut *mut int (relinquish_locality)(struct tpm_chip chip, int,
    pub value): *mut *mut *mut void (clk_enable)(struct tpm_chip chip, bool,
}

pub const TPM_NUM_EVENT_LOG_FILES: c_int = 3;
// Indexes the duration array
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tpm_duration {
    TPM_SHORT = 0,
    TPM_MEDIUM = 1,
    TPM_LONG = 2,
    TPM_LONG_LONG = 3,
    TPM_UNDEFINED,
    TPM_NUM_DURATIONS = TPM_UNDEFINED,
}

pub const TPM_PPI_VERSION_LEN: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tpm_space {
    pub context_tbl: [u32; 3],
    pub context_buf: *mut u8,
    pub session_tbl: [u32; 3],
    pub session_buf: *mut u8,
    pub buf_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tpm_bios_log {
    pub bios_event_log: *mut c_void,
    pub bios_event_log_end: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tpm_chip_seqops {
    pub chip: *mut tpm_chip,
    pub seqops: *const seq_operations,
}

// Fixed define for the curve we use which is NIST_P256
pub const EC_PT_SZ: c_int = 32;
//
// fixed define for the size of a name.  This is actually HASHALG size
// plus 2, so 32 for SHA256
//
pub const TPM2_NAME_SIZE: c_int = 34;
//
// The maximum size for an object context
//
pub const TPM2_MAX_CONTEXT_SIZE: c_int = 4096;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tpm_chip {
    pub dev: device,
    pub devs: device,
    pub cdev: cdev,
    pub cdevs: cdev,
// A driver callback under ops cannot be run unless ops_sem is held
// (sometimes implicitly, eg for the sysfs code). ops becomes null
// when the driver is unregistered, see tpm_try_get_ops.
//
    pub ops_sem: rw_semaphore,
    pub ops: *const tpm_class_ops,
    pub log: tpm_bios_log,
    pub bin_log_seqops: tpm_chip_seqops,
    pub ascii_log_seqops: tpm_chip_seqops,
    pub flags: c_uint,
    pub /: *mut *mut int dev_num; / /dev/tpm#,
    pub /: *mut *mut unsigned long is_open; / only one allowed,
    pub hwrng_name: [c_char; 64],
    pub hwrng: hwrng,
    pub /: *mut *mut mutex tpm_mutex; / tpm is processing,
    pub /: *mut *mut unsigned long timeout_a; / jiffies,
    pub /: *mut *mut unsigned long timeout_b; / jiffies,
    pub /: *mut *mut unsigned long timeout_c; / jiffies,
    pub /: *mut *mut unsigned long timeout_d; / jiffies,
    pub timeout_adjusted: bool,
    pub /: *mut *mut unsigned long duration[TPM_NUM_DURATIONS]; / jiffies,
    pub duration_adjusted: bool,
    pub bios_dir: *mut dentry,
    pub TPM_MAX_HASHES]: *const *const attribute_group groups[3 +,
    pub groups_cnt: c_uint,
    pub nr_allocated_banks: u32,
    pub allocated_banks: [tpm_bank_info; TPM2_MAX_PCR_BANKS],
    pub acpi_dev_handle: acpi_handle,
    pub 1]: char ppi_version[TPM_PPI_VERSION_LEN +,

    pub work_space: tpm_space,
    pub last_cc: u32,
    pub nr_commands: u32,
    pub cc_attrs_tbl: *mut u32,
// active locality
    pub locality: c_int,

// details for communication security via sessions
// saved context for NULL seed
    pub null_key_context: [u8; TPM2_MAX_CONTEXT_SIZE],
// name of NULL seed
    pub null_key_name: [u8; TPM2_NAME_SIZE],
    pub null_ec_key_x: [u8; EC_PT_SZ],
    pub null_ec_key_y: [u8; EC_PT_SZ],
    pub auth: *mut tpm2_auth,

}

pub const TPM_VID_INTEL: c_uint = 0x8086;
pub const TPM_VID_WINBOND: c_uint = 0x1050;
pub const TPM_VID_STM: c_uint = 0x104A;
pub const TPM_VID_ATML: c_uint = 0x1114;
pub const TPM_VID_IFX: c_uint = 0x15D1;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tpm_chip_flags {
    TPM_CHIP_FLAG_BOOTSTRAPPED		= BIT(0),
    TPM_CHIP_FLAG_TPM2			= BIT(1),
    TPM_CHIP_FLAG_IRQ			= BIT(2),
    TPM_CHIP_FLAG_VIRTUAL			= BIT(3),
    TPM_CHIP_FLAG_HAVE_TIMEOUTS		= BIT(4),
    TPM_CHIP_FLAG_ALWAYS_POWERED		= BIT(5),
    TPM_CHIP_FLAG_FIRMWARE_POWER_MANAGED	= BIT(6),
    TPM_CHIP_FLAG_FIRMWARE_UPGRADE		= BIT(7),
    TPM_CHIP_FLAG_SUSPENDED			= BIT(8),
    TPM_CHIP_FLAG_HWRNG_DISABLED		= BIT(9),
    TPM_CHIP_FLAG_DISABLE			= BIT(10),
    TPM_CHIP_FLAG_SYNC			= BIT(11),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tpm2_hash {
    pub crypto_id: c_uint,
    pub tpm_id: c_uint,
}

//
// Check if TPM device is in the firmware upgrade mode.
//
// Convert a return value from tpm_transmit_cmd() to POSIX error code.
//

extern "C" {
    pub fn tpm_is_tpm2(chip: *mut tpm_chip) -> c_int;
}
extern "C" {
    pub fn tpm_try_get_ops(chip: *mut tpm_chip) -> __must_check int;
}
extern "C" {
    pub fn tpm_put_ops(chip: *mut tpm_chip);
}
extern "C" {
    pub fn tpm_get_random(chip: *mut tpm_chip, data: *mut u8, max: usize) -> c_int;
}
extern "C" {
    pub fn tpm2_flush_context(chip: *mut tpm_chip, handle: u32);
}
extern "C" {
    pub fn tpm2_find_hash_alg(crypto_id: c_uint) -> c_int;
}
// simple authorization for empty auth

extern "C" {
    pub fn tpm2_start_auth_session(chip: *mut tpm_chip) -> c_int;
}
extern "C" {
    pub fn tpm_buf_fill_hmac_session(chip: *mut tpm_chip, buf: *mut tpm_buf) -> c_int;
}
extern "C" {
    pub fn tpm2_end_auth_session(chip: *mut tpm_chip);
}

