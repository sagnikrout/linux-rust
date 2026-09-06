//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/ethtool_netlink_generated.h
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


// SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR BSD-3-Clause)
// Do not edit directly, auto-generated from:
// Documentation/netlink/specs/ethtool.yaml
// YNL-GEN uapi header
// To regenerate run: tools/net/ynl/ynl-regen.sh

pub const ETHTOOL_GENL_VERSION: c_int = 1;
// private:
//
// enum ethtool_header_flags - common ethtool header flags
// @ETHTOOL_FLAG_COMPACT_BITSETS: use compact bitsets in reply
// @ETHTOOL_FLAG_OMIT_REPLY: provide optional reply for SET or ACT requests
// @ETHTOOL_FLAG_STATS: request statistics, if supported by the driver
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ethtool_header_flags {
    ETHTOOL_FLAG_COMPACT_BITSETS = 1,
    ETHTOOL_FLAG_OMIT_REPLY = 2,
    ETHTOOL_FLAG_STATS = 4,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ethtool_tcp_data_split {
    ETHTOOL_TCP_DATA_SPLIT_UNKNOWN,
    ETHTOOL_TCP_DATA_SPLIT_DISABLED,
    ETHTOOL_TCP_DATA_SPLIT_ENABLED,
}

//
// enum hwtstamp_source - Source of the hardware timestamp
// @HWTSTAMP_SOURCE_NETDEV: Hardware timestamp comes from a MAC or a device
// which has MAC and PHY integrated
// @HWTSTAMP_SOURCE_PHYLIB: Hardware timestamp comes from one PHY device of the
// network topology
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hwtstamp_source {
    HWTSTAMP_SOURCE_NETDEV = 1,
    HWTSTAMP_SOURCE_PHYLIB,
}

//
// enum ethtool_pse_event - PSE event list for the PSE controller
// @ETHTOOL_PSE_EVENT_OVER_CURRENT: PSE output current is too high
// @ETHTOOL_PSE_EVENT_OVER_TEMP: PSE in over temperature state
// @ETHTOOL_C33_PSE_EVENT_DETECTION: detection process occur on the PSE. IEEE
// 802.3-2022 33.2.5 and 145.2.6 PSE detection of PDs. IEEE 802.3-202
// 30.9.1.1.5 aPSEPowerDetectionStatus
// @ETHTOOL_C33_PSE_EVENT_CLASSIFICATION: classification process occur on the
// PSE. IEEE 802.3-2022 33.2.6 and 145.2.8 classification of PDs mutual
// identification. IEEE 802.3-2022 30.9.1.1.8 aPSEPowerClassification.
// @ETHTOOL_C33_PSE_EVENT_DISCONNECTION: PD has been disconnected on the PSE.
// IEEE 802.3-2022 33.3.8 and 145.3.9 PD Maintain Power Signature. IEEE
// 802.3-2022 33.5.1.2.9 MPS Absent. IEEE 802.3-2022 30.9.1.1.20
// aPSEMPSAbsentCounter.
// @ETHTOOL_PSE_EVENT_OVER_BUDGET: PSE turned off due to over budget situation
// @ETHTOOL_PSE_EVENT_SW_PW_CONTROL_ERROR: PSE faced an error managing the
// power control from software
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ethtool_pse_event {
    ETHTOOL_PSE_EVENT_OVER_CURRENT = 1,
    ETHTOOL_PSE_EVENT_OVER_TEMP = 2,
    ETHTOOL_C33_PSE_EVENT_DETECTION = 4,
    ETHTOOL_C33_PSE_EVENT_CLASSIFICATION = 8,
    ETHTOOL_C33_PSE_EVENT_DISCONNECTION = 16,
    ETHTOOL_PSE_EVENT_OVER_BUDGET = 32,
    ETHTOOL_PSE_EVENT_SW_PW_CONTROL_ERROR = 64,
}

