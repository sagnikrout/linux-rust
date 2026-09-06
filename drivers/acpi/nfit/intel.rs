//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/acpi/nfit/intel.h
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
//
// Copyright(c) 2018 Intel Corporation. All rights reserved.
// Intel specific definitions for NVDIMM Firmware Interface Table - NFIT
//
pub const ND_INTEL_SMART: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nd_intel_smart {
    pub status: u32,
    pub flags: u32,
    pub reserved0: [u8; 4],
    pub health: u8,
    pub spares: u8,
    pub life_used: u8,
    pub alarm_flags: u8,
    pub media_temperature: u16,
    pub ctrl_temperature: u16,
    pub shutdown_count: u32,
    pub ait_status: u8,
    pub pmic_temperature: u16,
    pub reserved1: [u8; 8],
    pub shutdown_state: u8,
    pub vendor_size: u32,
    pub vendor_data: [u8; 92],
    pub __packed: },
    pub data: [u8; 128],
}

pub const ND_INTEL_STATUS_SIZE: c_int = 4;
pub const ND_INTEL_PASSPHRASE_SIZE: c_int = 32;
pub const ND_INTEL_STATUS_NOT_SUPPORTED: c_int = 1;
pub const ND_INTEL_STATUS_RETRY: c_int = 5;
pub const ND_INTEL_STATUS_NOT_READY: c_int = 9;
pub const ND_INTEL_STATUS_INVALID_STATE: c_int = 10;
pub const ND_INTEL_STATUS_INVALID_PASS: c_int = 11;
pub const ND_INTEL_STATUS_OVERWRITE_UNSUPPORTED: c_uint = 0x10007;
pub const ND_INTEL_STATUS_OQUERY_INPROGRESS: c_uint = 0x10007;
pub const ND_INTEL_STATUS_OQUERY_SEQUENCE_ERR: c_uint = 0x20007;
pub const ND_INTEL_SEC_STATE_ENABLED: c_uint = 0x02;
pub const ND_INTEL_SEC_STATE_LOCKED: c_uint = 0x04;
pub const ND_INTEL_SEC_STATE_FROZEN: c_uint = 0x08;
pub const ND_INTEL_SEC_STATE_PLIMIT: c_uint = 0x10;
pub const ND_INTEL_SEC_STATE_UNSUPPORTED: c_uint = 0x20;
pub const ND_INTEL_SEC_STATE_OVERWRITE: c_uint = 0x40;
pub const ND_INTEL_SEC_ESTATE_ENABLED: c_uint = 0x01;
pub const ND_INTEL_SEC_ESTATE_PLIMIT: c_uint = 0x02;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nd_intel_get_security_state {
    pub status: u32,
    pub extended_state: u8,
    pub reserved: [u8; 3],
    pub state: u8,
    pub reserved1: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nd_intel_set_passphrase {
    pub old_pass: [u8; ND_INTEL_PASSPHRASE_SIZE],
    pub new_pass: [u8; ND_INTEL_PASSPHRASE_SIZE],
    pub status: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nd_intel_unlock_unit {
    pub passphrase: [u8; ND_INTEL_PASSPHRASE_SIZE],
    pub status: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nd_intel_disable_passphrase {
    pub passphrase: [u8; ND_INTEL_PASSPHRASE_SIZE],
    pub status: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nd_intel_freeze_lock {
    pub status: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nd_intel_secure_erase {
    pub passphrase: [u8; ND_INTEL_PASSPHRASE_SIZE],
    pub status: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nd_intel_overwrite {
    pub passphrase: [u8; ND_INTEL_PASSPHRASE_SIZE],
    pub status: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nd_intel_query_overwrite {
    pub status: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nd_intel_set_master_passphrase {
    pub old_pass: [u8; ND_INTEL_PASSPHRASE_SIZE],
    pub new_pass: [u8; ND_INTEL_PASSPHRASE_SIZE],
    pub status: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nd_intel_master_secure_erase {
    pub passphrase: [u8; ND_INTEL_PASSPHRASE_SIZE],
    pub status: u32,
    pub __packed: },
pub const ND_INTEL_FWA_IDLE: c_int = 0;
pub const ND_INTEL_FWA_ARMED: c_int = 1;
pub const ND_INTEL_FWA_BUSY: c_int = 2;
pub const ND_INTEL_DIMM_FWA_NONE: c_int = 0;
pub const ND_INTEL_DIMM_FWA_NOTSTAGED: c_int = 1;
pub const ND_INTEL_DIMM_FWA_SUCCESS: c_int = 2;
pub const ND_INTEL_DIMM_FWA_NEEDRESET: c_int = 3;
pub const ND_INTEL_DIMM_FWA_MEDIAFAILED: c_int = 4;
pub const ND_INTEL_DIMM_FWA_ABORT: c_int = 5;
pub const ND_INTEL_DIMM_FWA_NOTSUPP: c_int = 6;
pub const ND_INTEL_DIMM_FWA_ERROR: c_int = 7;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nd_intel_fw_activate_dimminfo {
    pub status: u32,
    pub result: u16,
    pub state: u8,
    pub reserved: [u8; 7],
    pub __packed: },
pub const ND_INTEL_DIMM_FWA_ARM: c_int = 1;
pub const ND_INTEL_DIMM_FWA_DISARM: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nd_intel_fw_activate_arm {
    pub activate_arm: u8,
    pub status: u32,
    pub __packed: },
// Root device command payloads

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nd_intel_bus_fw_activate_businfo {
    pub status: u32,
    pub reserved: u16,
    pub state: u8,
    pub capability: u8,
    pub activate_tmo: u64,
    pub cpu_quiesce_tmo: u64,
    pub io_quiesce_tmo: u64,
    pub max_quiesce_tmo: u64,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nd_intel_bus_fw_activate {
    pub iodev_state: u8,
    pub status: u32,
    pub __packed: },
    pub intel_fw_ops: *const extern struct nvdimm_fw_ops,
    pub intel_bus_fw_ops: *const extern struct nvdimm_bus_fw_ops,
