//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/meta/fbnic/fbnic_mac.h
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

// The RXB clock runs at 600 MHZ in the ASIC and the PAUSE_STORM_UNIT_WR
// is 10us granularity, so set the clock to 6000 (0x1770)
//
pub const FBNIC_RXB_PS_CLK_DIV: c_uint = 0x1770;
// Convert milliseconds to pause storm timeout units (10us granularity)

// Convert pause storm timeout units (10us granularity) to milliseconds

// Set the default timer to 500ms, which should be longer than any
// reasonable period of continuous pausing. The service task, which runs
// once per second, periodically resets the pause storm trigger.
//
// As a result, on a functioning system, if pause continues, we enforce
// a duty cycle determined by the configured pause storm timeout (50%
// default). A crashed system will not have the service task and therefore
// pause will remain disabled until reboot recovery.
//
pub const FBNIC_MAC_PS_TO_DEFAULT_MS: c_int = 500;

pub const FBNIC_MAX_JUMBO_FRAME_SIZE: c_int = 9742;
// States loosely based on section 136.8.11.7.5 of IEEE 802.3-2022 Ethernet
// Standard.  These are needed to track the state of the PHY as it has a delay
// of several seconds from the time link comes up until it has completed
// training that we need to wait to report the link.
//
// Currently we treat training as a single block as this is managed by the
// firmware.
//
// We have FBNIC_PMD_SEND_DATA set to 0 as the expected default at driver load
// and we initialize the structure containing it to zero at allocation.
//
// Treat the FEC bits as a bitmask laid out as follows:
// Bit 0: RS Enabled
// Bit 1: BASER(Firecode) Enabled
// Bit 2: Retrieve FEC from FW
//
// Treat the AUI modes as a modulation/lanes bitmask:
// Bit 0: Lane Count, 0 = R1, 1 = R2
// Bit 1: Modulation, 0 = NRZ, 1 = PAM4
// Bit 2: Unknown Modulation/Lane Configuration
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fbnic_sensor_id {
    FBNIC_SENSOR_TEMP,		/* Temp in millidegrees Centigrade */
    FBNIC_SENSOR_VOLTAGE,		/* Voltage in millivolts */
}

// This structure defines the interface hooks for the MAC. The MAC hooks
// will be configured as a const struct provided with a set of function
// pointers.
//
// void (*init_regs)(struct fbnic_dev *fbd);
// Initialize MAC registers to enable Tx/Rx paths and FIFOs.
//
// int (*get_link_event)(struct fbnic_dev *fbd)
// Get the current link event status, reports true if link has
// changed to either FBNIC_LINK_EVENT_DOWN or FBNIC_LINK_EVENT_UP
// bool (*get_link)(struct fbnic_dev *fbd, u8 aui, u8 fec);
// Check link status
//
// void (*prepare)(struct fbnic_dev *fbd, u8 aui, u8 fec);
// Prepare PHY for init by fetching settings, disabling interrupts,
// and sending an updated PHY config to FW if needed.
//
// void (*link_down)(struct fbnic_dev *fbd);
// Configure MAC for link down event
// void (*link_up)(struct fbnic_dev *fbd, bool tx_pause, bool rx_pause);
// Configure MAC for link up event;
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fbnic_mac {
    pub fbd): *mut *mut void (init_regs)(struct fbnic_dev,
    pub fbd): *mut *mut int (get_link_event)(struct fbnic_dev,
    pub fec): *mut *mut *mut bool (get_link)(struct fbnic_dev fbd, u8 aui, u8,
    pub fec): *mut *mut *mut void (prepare)(struct fbnic_dev fbd, u8 aui, u8,
    pub fec_stats): *mut fbnic_fec_stats,
    pub pcs_stats): *mut fbnic_pcs_stats,
    pub mac_stats): *mut fbnic_eth_mac_stats,
    pub pause_stats): *mut fbnic_pause_stats,
    pub ctrl_stats): *mut fbnic_eth_ctrl_stats,
    pub rmon_stats): *mut fbnic_rmon_stats,
    pub fbd): *mut *mut void (link_down)(struct fbnic_dev,
    pub rx_pause): *mut *mut *mut void (link_up)(struct fbnic_dev fbd, bool tx_pause, bool,
    pub val): *mut *mut *mut int (get_sensor)(struct fbnic_dev fbd, int id, long,
}

extern "C" {
    pub fn fbnic_mac_init(fbd: *mut fbnic_dev) -> c_int;
}
extern "C" {
    pub fn fbnic_mac_get_fw_settings(fbd: *mut fbnic_dev, aui: *mut u8, fec: *mut u8);
}
extern "C" {
    pub fn fbnic_mac_ps_protect_to_config(fbd: *mut fbnic_dev, timeout: u16) -> c_int;
}
extern "C" {
    pub fn fbnic_mac_ps_protect_handler(fbd: *mut fbnic_dev);
}
extern "C" {
    pub fn fbnic_mac_check_tx_pause(fbd: *mut fbnic_dev) -> bool;
}
