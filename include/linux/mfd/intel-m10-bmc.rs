//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/intel-m10-bmc.h
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
// Intel MAX 10 Board Management Controller chip.
//
// Copyright (C) 2018-2020 Intel Corporation, Inc.
//

pub const M10BMC_N3000_LEGACY_BUILD_VER: c_uint = 0x300468;
pub const M10BMC_N3000_SYS_BASE: c_uint = 0x300800;
pub const M10BMC_N3000_SYS_END: c_uint = 0x300fff;
pub const M10BMC_N3000_FLASH_BASE: c_uint = 0x10000000;
pub const M10BMC_N3000_FLASH_END: c_uint = 0x1fffffff;

pub const M10BMC_STAGING_BASE: c_uint = 0x18000000;
pub const M10BMC_STAGING_SIZE: c_uint = 0x3800000;
// Register offset of system registers
pub const NIOS2_N3000_FW_VERSION: c_uint = 0x0;
pub const M10BMC_N3000_MAC_LOW: c_uint = 0x10;

pub const M10BMC_N3000_MAC_HIGH: c_uint = 0x14;

pub const M10BMC_N3000_TEST_REG: c_uint = 0x3c;
pub const M10BMC_N3000_BUILD_VER: c_uint = 0x68;

pub const M10BMC_N3000_VER_LEGACY_INVALID: c_uint = 0xffffffff;
// Telemetry registers
pub const M10BMC_N3000_TELEM_START: c_uint = 0x100;
pub const M10BMC_N3000_TELEM_END: c_uint = 0x250;
pub const M10BMC_D5005_TELEM_END: c_uint = 0x300;
// Secure update doorbell register, in system register region
pub const M10BMC_N3000_DOORBELL: c_uint = 0x400;
// Authorization Result register, in system register region
pub const M10BMC_N3000_AUTH_RESULT: c_uint = 0x404;
// Doorbell register fields

// Progress states
pub const RSU_PROG_IDLE: c_uint = 0x0;
pub const RSU_PROG_PREPARE: c_uint = 0x1;
pub const RSU_PROG_READY: c_uint = 0x3;
pub const RSU_PROG_AUTHENTICATING: c_uint = 0x4;
pub const RSU_PROG_COPYING: c_uint = 0x5;
pub const RSU_PROG_UPDATE_CANCEL: c_uint = 0x6;
pub const RSU_PROG_PROGRAM_KEY_HASH: c_uint = 0x7;
pub const RSU_PROG_RSU_DONE: c_uint = 0x8;
pub const RSU_PROG_PKVL_PROM_DONE: c_uint = 0x9;
// Device and error states
pub const RSU_STAT_NORMAL: c_uint = 0x0;
pub const RSU_STAT_TIMEOUT: c_uint = 0x1;
pub const RSU_STAT_AUTH_FAIL: c_uint = 0x2;
pub const RSU_STAT_COPY_FAIL: c_uint = 0x3;
pub const RSU_STAT_FATAL: c_uint = 0x4;
pub const RSU_STAT_PKVL_REJECT: c_uint = 0x5;
pub const RSU_STAT_NON_INC: c_uint = 0x6;
pub const RSU_STAT_ERASE_FAIL: c_uint = 0x7;
pub const RSU_STAT_WEAROUT: c_uint = 0x8;
pub const RSU_STAT_NIOS_OK: c_uint = 0x80;
pub const RSU_STAT_USER_OK: c_uint = 0x81;
pub const RSU_STAT_FACTORY_OK: c_uint = 0x82;
pub const RSU_STAT_USER_FAIL: c_uint = 0x83;
pub const RSU_STAT_FACTORY_FAIL: c_uint = 0x84;
pub const RSU_STAT_NIOS_FLASH_ERR: c_uint = 0x85;
pub const RSU_STAT_FPGA_FLASH_ERR: c_uint = 0x86;
pub const HOST_STATUS_IDLE: c_uint = 0x0;
pub const HOST_STATUS_WRITE_DONE: c_uint = 0x1;
pub const HOST_STATUS_ABORT_RSU: c_uint = 0x2;

// interval 100ms and timeout 5s

// RSU PREP Timeout (2 minutes) to erase flash staging area
pub const RSU_PREP_INTERVAL_MS: c_int = 100;

// RSU Complete Timeout (40 minutes) for full flash update
pub const RSU_COMPLETE_INTERVAL_MS: c_int = 1000;

// Addresses for security related data in FLASH
pub const M10BMC_N3000_BMC_REH_ADDR: c_uint = 0x17ffc004;
pub const M10BMC_N3000_BMC_PROG_ADDR: c_uint = 0x17ffc000;
pub const M10BMC_N3000_BMC_PROG_MAGIC: c_uint = 0x5746;
pub const M10BMC_N3000_SR_REH_ADDR: c_uint = 0x17ffd004;
pub const M10BMC_N3000_SR_PROG_ADDR: c_uint = 0x17ffd000;
pub const M10BMC_N3000_SR_PROG_MAGIC: c_uint = 0x5253;
pub const M10BMC_N3000_PR_REH_ADDR: c_uint = 0x17ffe004;
pub const M10BMC_N3000_PR_PROG_ADDR: c_uint = 0x17ffe000;
pub const M10BMC_N3000_PR_PROG_MAGIC: c_uint = 0x5250;
// Address of 4KB inverted bit vector containing staging area FLASH count
pub const M10BMC_N3000_STAGING_FLASH_COUNT: c_uint = 0x17ffb000;
pub const M10BMC_N6000_INDIRECT_BASE: c_uint = 0x400;
pub const M10BMC_N6000_SYS_BASE: c_uint = 0x0;
pub const M10BMC_N6000_SYS_END: c_uint = 0xfff;
pub const M10BMC_N6000_DOORBELL: c_uint = 0x1c0;
pub const M10BMC_N6000_AUTH_RESULT: c_uint = 0x1c4;

pub const M10BMC_N6000_BUILD_VER: c_uint = 0x0;
pub const NIOS2_N6000_FW_VERSION: c_uint = 0x4;
pub const M10BMC_N6000_MAC_LOW: c_uint = 0x20;

// Addresses for security related data in FLASH
pub const M10BMC_N6000_BMC_REH_ADDR: c_uint = 0x7ffc004;
pub const M10BMC_N6000_BMC_PROG_ADDR: c_uint = 0x7ffc000;
pub const M10BMC_N6000_BMC_PROG_MAGIC: c_uint = 0x5746;
pub const M10BMC_N6000_SR_REH_ADDR: c_uint = 0x7ffd004;
pub const M10BMC_N6000_SR_PROG_ADDR: c_uint = 0x7ffd000;
pub const M10BMC_N6000_SR_PROG_MAGIC: c_uint = 0x5253;
pub const M10BMC_N6000_PR_REH_ADDR: c_uint = 0x7ffe004;
pub const M10BMC_N6000_PR_PROG_ADDR: c_uint = 0x7ffe000;
pub const M10BMC_N6000_PR_PROG_MAGIC: c_uint = 0x5250;
pub const M10BMC_N6000_STAGING_FLASH_COUNT: c_uint = 0x7ff5000;
pub const M10BMC_N6000_FLASH_MUX_CTRL: c_uint = 0x1d0;

pub const M10BMC_N6000_FLASH_MUX_IDLE: c_int = 0;
pub const M10BMC_N6000_FLASH_MUX_NIOS: c_int = 1;
pub const M10BMC_N6000_FLASH_MUX_HOST: c_int = 2;
pub const M10BMC_N6000_FLASH_MUX_PFL: c_int = 4;

pub const M10BMC_N6000_FLASH_CTRL: c_uint = 0x40;

pub const M10BMC_N6000_FLASH_ADDR: c_uint = 0x44;
pub const M10BMC_N6000_FLASH_FIFO: c_uint = 0x800;
pub const M10BMC_N6000_READ_BLOCK_SIZE: c_uint = 0x800;
pub const M10BMC_N6000_FIFO_MAX_BYTES: c_uint = 0x800;
pub const M10BMC_N6000_FIFO_WORD_SIZE: c_int = 4;

pub const M10BMC_FLASH_INT_US: c_int = 1;
pub const M10BMC_FLASH_TIMEOUT_US: c_int = 10000;
//
// struct m10bmc_csr_map - Intel MAX 10 BMC CSR register map
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct m10bmc_csr_map {
    pub base: c_uint,
    pub build_version: c_uint,
    pub fw_version: c_uint,
    pub mac_low: c_uint,
    pub mac_high: c_uint,
    pub doorbell: c_uint,
    pub auth_result: c_uint,
    pub bmc_prog_addr: c_uint,
    pub bmc_reh_addr: c_uint,
    pub bmc_magic: c_uint,
    pub sr_prog_addr: c_uint,
    pub sr_reh_addr: c_uint,
    pub sr_magic: c_uint,
    pub pr_prog_addr: c_uint,
    pub pr_reh_addr: c_uint,
    pub pr_magic: c_uint,
    pub rsu_update_counter: c_uint,
    pub staging_size: c_uint,
}

//
// struct intel_m10bmc_platform_info - Intel MAX 10 BMC platform specific information
// @cells: MFD cells
// @n_cells: MFD cells ARRAY_SIZE()
// @handshake_sys_reg_ranges: array of register ranges for fw handshake regs
// @handshake_sys_reg_nranges: number of register ranges for fw handshake regs
// @csr_map: the mappings for register definition of MAX10 BMC
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_m10bmc_platform_info {
    pub cells: *mut mfd_cell,
    pub n_cells: c_int,
    pub handshake_sys_reg_ranges: *const regmap_range,
    pub handshake_sys_reg_nranges: c_uint,
    pub csr_map: *const m10bmc_csr_map,
}

//
// struct intel_m10bmc_flash_bulk_ops - device specific operations for flash R/W
// @read: read a block of data from flash
// @write: write a block of data to flash
// @lock_write: locks flash access for erase+write
// @unlock_write: unlock flash access
//
// Write must be protected with @lock_write and @unlock_write. While the flash
// is locked, @read returns -EBUSY.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_m10bmc_flash_bulk_ops {
    pub size): *mut *mut *mut *mut int (read)(struct intel_m10bmc m10bmc, u8 buf, u32 addr, u32,
    pub size): *const *const *const *const int (write)(struct intel_m10bmc m10bmc, u8 buf, u32 offset, u32,
    pub m10bmc): *mut *mut int (lock_write)(struct intel_m10bmc,
    pub m10bmc): *mut *mut void (unlock_write)(struct intel_m10bmc,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum m10bmc_fw_state {
    M10BMC_FW_STATE_NORMAL,
    M10BMC_FW_STATE_SEC_UPDATE_PREPARE,
    M10BMC_FW_STATE_SEC_UPDATE_WRITE,
    M10BMC_FW_STATE_SEC_UPDATE_PROGRAM,
}

//
// struct intel_m10bmc - Intel MAX 10 BMC parent driver data structure
// @dev: this device
// @regmap: the regmap used to access registers by m10bmc itself
// @info: the platform information for MAX10 BMC
// @flash_bulk_ops: optional device specific operations for flash R/W
// @bmcfw_lock: read/write semaphore to BMC firmware running state
// @bmcfw_state: BMC firmware running state. Available only when
// handshake_sys_reg_nranges > 0.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_m10bmc {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub info: *const intel_m10bmc_platform_info,
    pub flash_bulk_ops: *const intel_m10bmc_flash_bulk_ops,
    pub /: *mut *mut rw_semaphore bmcfw_lock; / Protects bmcfw_state,
    pub bmcfw_state: m10bmc_fw_state,
}

//
// register access helper functions.
//
// m10bmc_raw_read - read m10bmc register per addr
// m10bmc_sys_read - read m10bmc system register per offset
// m10bmc_sys_update_bits - update m10bmc system register per offset
//
extern "C" {
    pub fn m10bmc_sys_read(m10bmc: *mut intel_m10bmc, offset: c_uint, val: *mut c_uint) -> c_int;
}
//
// Track the state of the firmware, as it is not available for register
// handshakes during secure updates on some MAX 10 cards.
//
extern "C" {
    pub fn m10bmc_fw_state_set(m10bmc: *mut intel_m10bmc, new_state: m10bmc_fw_state);
}
//
// MAX10 BMC Core support
//
extern "C" {
    pub fn m10bmc_dev_init(m10bmc: *mut intel_m10bmc, info: *const intel_m10bmc_platform_info) -> c_int;
}
