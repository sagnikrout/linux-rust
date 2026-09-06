//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/bluetooth/hci_mon.h
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_mon_hdr {
    pub opcode: __le16,
    pub index: __le16,
    pub len: __le16,
    pub __packed: },
pub const HCI_MON_HDR_SIZE: c_int = 6;
pub const HCI_MON_NEW_INDEX: c_int = 0;
pub const HCI_MON_DEL_INDEX: c_int = 1;
pub const HCI_MON_COMMAND_PKT: c_int = 2;
pub const HCI_MON_EVENT_PKT: c_int = 3;
pub const HCI_MON_ACL_TX_PKT: c_int = 4;
pub const HCI_MON_ACL_RX_PKT: c_int = 5;
pub const HCI_MON_SCO_TX_PKT: c_int = 6;
pub const HCI_MON_SCO_RX_PKT: c_int = 7;
pub const HCI_MON_OPEN_INDEX: c_int = 8;
pub const HCI_MON_CLOSE_INDEX: c_int = 9;
pub const HCI_MON_INDEX_INFO: c_int = 10;
pub const HCI_MON_VENDOR_DIAG: c_int = 11;
pub const HCI_MON_SYSTEM_NOTE: c_int = 12;
pub const HCI_MON_USER_LOGGING: c_int = 13;
pub const HCI_MON_CTRL_OPEN: c_int = 14;
pub const HCI_MON_CTRL_CLOSE: c_int = 15;
pub const HCI_MON_CTRL_COMMAND: c_int = 16;
pub const HCI_MON_CTRL_EVENT: c_int = 17;
pub const HCI_MON_ISO_TX_PKT: c_int = 18;
pub const HCI_MON_ISO_RX_PKT: c_int = 19;
pub const HCI_MON_DRV_TX_PKT: c_int = 20;
pub const HCI_MON_DRV_RX_PKT: c_int = 21;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_mon_new_index {
    pub type: __u8,
    pub bus: __u8,
    pub bdaddr: bdaddr_t,
    pub __nonstring: char name[8],
    pub __packed: },
pub const HCI_MON_NEW_INDEX_SIZE: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_mon_index_info {
    pub bdaddr: bdaddr_t,
    pub manufacturer: __le16,
    pub __packed: },
pub const HCI_MON_INDEX_INFO_SIZE: c_int = 8;
