//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/meta/fbnic/fbnic.h
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
// Copyright (c) Meta Platforms, Inc. and affiliates.

pub const FBNIC_MBX_CMPL_SLOTS: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fbnic_dev {
    pub dev: *mut device,
    pub netdev: *mut net_device,
    pub dbg_fbd: *mut dentry,
    pub hwmon: *mut device,
    pub fw_reporter: *mut devlink_health_reporter,
    pub otp_reporter: *mut devlink_health_reporter,
    pub uc_addr0: *mut u32 __iomem,
    pub uc_addr4: *mut u32 __iomem,
    pub mac: *const fbnic_mac,
    pub fw_msix_vector: c_uint,
    pub mac_msix_vector: c_uint,
    pub num_irqs: c_ushort,
    pub users: u8,
    pub 9]: char name[IFNAMSIZ +,
    pub napi_irq: [}; FBNIC_MAX_NAPI_VECTORS],
    pub service_task: delayed_work,
    pub mbx: [fbnic_fw_mbx; FBNIC_IPC_MBX_INDICES],
    pub fw_cap: fbnic_fw_cap,
    pub cmpl_data: [*mut fbnic_fw_completion; FBNIC_MBX_CMPL_SLOTS],
// Lock protecting Tx Mailbox queue to prevent possible races
    pub fw_tx_lock: spinlock_t,
    pub last_heartbeat_request: c_ulong,
    pub last_heartbeat_response: c_ulong,
    pub fw_heartbeat_enabled: u8,
    pub dsn: u64,
    pub mps: u32,
    pub readrq: u32,
    pub relaxed_ord: u8,
// Local copy of the devices TCAM
    pub act_tcam: [fbnic_act_tcam; FBNIC_RPC_TCAM_ACT_NUM_ENTRIES],
    pub mac_addr: [fbnic_mac_addr; FBNIC_RPC_TCAM_MACDA_NUM_ENTRIES],
    pub mac_addr_boundary: u8,
    pub tce_tcam_last: u8,
// IP TCAM
    pub ip_src: [fbnic_ip_addr; FBNIC_RPC_TCAM_IP_ADDR_NUM_ENTRIES],
    pub ip_dst: [fbnic_ip_addr; FBNIC_RPC_TCAM_IP_ADDR_NUM_ENTRIES],
    pub ipo_src: [fbnic_ip_addr; FBNIC_RPC_TCAM_IP_ADDR_NUM_ENTRIES],
    pub ipo_dst: [fbnic_ip_addr; FBNIC_RPC_TCAM_IP_ADDR_NUM_ENTRIES],
// Number of TCQs/RCQs available on hardware
    pub max_num_queues: u16,
// Lock protecting writes to @time_high, @time_offset of fbnic_netdev,
// and the HW time CSR machinery.
//
    pub time_lock: spinlock_t,
// Externally accessible PTP clock, may be NULL
    pub ptp: *mut ptp_clock,
    pub ptp_info: ptp_clock_info,
// Last @time_high refresh time in jiffies (to catch stalls)
    pub last_read: c_ulong,
// PMD specific data
    pub end_of_pmd_training: c_ulong,
    pub pmd_state: u8,
// Local copy of hardware statistics
    pub hw_stats: fbnic_hw_stats,
// Firmware time since boot in milliseconds
    pub firmware_time: u64,
    pub prev_firmware_time: u64,
    pub fw_log: fbnic_fw_log,
// MDIO bus for PHYs
    pub mdio_bus: *mut mii_bus,
// In units of ms since API supports values in ms
    pub ps_timeout: u16,
}

// Reserve entry 0 in the MSI-X "others" array until we have filled all
// 32 of the possible interrupt slots. By doing this we can avoid any
// potential conflicts should we need to enable one of the debug interrupt
// causes later.
//
extern "C" {
    pub fn fbnic_rd32(fbd: *mut fbnic_dev, reg: u32) -> u32;
}

extern "C" {
    pub fn fbnic_fw_present(fbd: *mut fbnic_dev) -> bool;
}
extern "C" {
    pub fn fbnic_fw_rd32(fbd: *mut fbnic_dev, reg: u32) -> u32;
}
extern "C" {
    pub fn fbnic_fw_wr32(fbd: *mut fbnic_dev, reg: u32, val: u32);
}

extern "C" {
    pub fn fbnic_devlink_free(fbd: *mut fbnic_dev);
}
extern "C" {
    pub fn fbnic_devlink_health_create(fbd: *mut fbnic_dev) -> c_int;
}
extern "C" {
    pub fn fbnic_devlink_health_destroy(fbd: *mut fbnic_dev);
}
extern "C" {
    pub fn fbnic_devlink_register(fbd: *mut fbnic_dev);
}
extern "C" {
    pub fn fbnic_devlink_unregister(fbd: *mut fbnic_dev);
}
extern "C" {
    pub fn fbnic_devlink_otp_check(fbd: *mut fbnic_dev, msg: *const c_char);
}
extern "C" {
    pub fn fbnic_fw_request_mbx(fbd: *mut fbnic_dev) -> c_int;
}
extern "C" {
    pub fn fbnic_fw_free_mbx(fbd: *mut fbnic_dev);
}
extern "C" {
    pub fn fbnic_hwmon_register(fbd: *mut fbnic_dev);
}
extern "C" {
    pub fn fbnic_hwmon_unregister(fbd: *mut fbnic_dev);
}
extern "C" {
    pub fn fbnic_mac_request_irq(fbd: *mut fbnic_dev) -> c_int;
}
extern "C" {
    pub fn fbnic_mac_free_irq(fbd: *mut fbnic_dev);
}
extern "C" {
    pub fn fbnic_napi_name_irqs(fbd: *mut fbnic_dev);
}
extern "C" {
    pub fn fbnic_synchronize_irq(fbd: *mut fbnic_dev, nr: c_int);
}
extern "C" {
    pub fn fbnic_free_irq(dev: *mut fbnic_dev, nr: c_int, data: *mut c_void);
}
//
// enum fbnic_msix_self_test_codes - return codes from self test routines
//
// These are the codes returned from the self test routines and
// stored in the test result array indexed by the specific
// test name.
//
// @FBNIC_TEST_MSIX_SUCCESS: no errors
// @FBNIC_TEST_MSIX_NOMEM: allocation failure
// @FBNIC_TEST_MSIX_IRQ_REQ_FAIL: IRQ request failure
// @FBNIC_TEST_MSIX_MASK: masking failed to prevent IRQ
// @FBNIC_TEST_MSIX_UNMASK: unmasking failure w/ sw status set
// @FBNIC_TEST_MSIX_IRQ_CLEAR: interrupt when clearing mask
// @FBNIC_TEST_MSIX_NO_INTERRUPT: no interrupt when not masked
// @FBNIC_TEST_MSIX_NO_CLEAR_OR_MASK: status not cleared, or mask not set
// @FBNIC_TEST_MSIX_BITS_SET_AFTER_TEST: Bits are set after test
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fbnic_msix_self_test_codes {
    FBNIC_TEST_MSIX_SUCCESS = 0,
    FBNIC_TEST_MSIX_NOMEM = 5,
    FBNIC_TEST_MSIX_IRQ_REQ_FAIL = 10,
    FBNIC_TEST_MSIX_MASK = 20,
    FBNIC_TEST_MSIX_UNMASK = 30,
    FBNIC_TEST_MSIX_IRQ_CLEAR = 40,
    FBNIC_TEST_MSIX_NO_INTERRUPT = 50,
    FBNIC_TEST_MSIX_NO_CLEAR_OR_MASK = 60,
    FBNIC_TEST_MSIX_BITS_SET_AFTER_TEST = 70,
}

extern "C" {
    pub fn fbnic_msix_test(fbd: *mut fbnic_dev) -> fbnic_msix_self_test_codes;
}
extern "C" {
    pub fn fbnic_free_irqs(fbd: *mut fbnic_dev);
}
extern "C" {
    pub fn fbnic_alloc_irqs(fbd: *mut fbnic_dev) -> c_int;
}
extern "C" {
    pub fn fbnic_dbg_fbd_init(fbd: *mut fbnic_dev);
}
extern "C" {
    pub fn fbnic_dbg_fbd_exit(fbd: *mut fbnic_dev);
}
extern "C" {
    pub fn fbnic_dbg_init();
}
extern "C" {
    pub fn fbnic_dbg_exit();
}
extern "C" {
    pub fn fbnic_rpc_reset_valid_entries(fbd: *mut fbnic_dev);
}
extern "C" {
    pub fn fbnic_mdiobus_create(fbd: *mut fbnic_dev) -> c_int;
}
extern "C" {
    pub fn fbnic_csr_get_regs(fbd: *mut fbnic_dev, data: *mut u32, regs_version: *mut u32);
}
extern "C" {
    pub fn fbnic_csr_regs_len(fbd: *mut fbnic_dev) -> c_int;
}
extern "C" {
    pub fn fbnic_config_txrx_usecs(nv: *mut fbnic_napi_vector, arm: u32);
}
extern "C" {
    pub fn fbnic_config_rx_frames(nv: *mut fbnic_napi_vector);
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fbnic_boards {
    fbnic_board_asic
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fbnic_info {
    pub max_num_queues: c_uint,
    pub bar_mask: c_uint,
}
