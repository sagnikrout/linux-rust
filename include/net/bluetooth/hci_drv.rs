//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/bluetooth/hci_drv.h
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
// Copyright (C) 2025 Google Corporation
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_drv_cmd_hdr {
    pub opcode: __le16,
    pub len: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_drv_ev_hdr {
    pub opcode: __le16,
    pub len: __le16,
    pub __packed: },
pub const HCI_DRV_EV_CMD_STATUS: c_uint = 0x0000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_drv_ev_cmd_status {
    pub opcode: __le16,
    pub status: __u8,
    pub __packed: },
pub const HCI_DRV_EV_CMD_COMPLETE: c_uint = 0x0001;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_drv_ev_cmd_complete {
    pub opcode: __le16,
    pub status: __u8,
    pub data: [__u8; ],
    pub __packed: },
pub const HCI_DRV_STATUS_SUCCESS: c_uint = 0x00;
pub const HCI_DRV_STATUS_UNSPECIFIED_ERROR: c_uint = 0x01;
pub const HCI_DRV_STATUS_UNKNOWN_COMMAND: c_uint = 0x02;
pub const HCI_DRV_STATUS_INVALID_PARAMETERS: c_uint = 0x03;
pub const HCI_DRV_MAX_DRIVER_NAME_LENGTH: c_int = 32;
// Common commands that make sense on all drivers start from 0x0000
pub const HCI_DRV_OP_READ_INFO: c_uint = 0x0000;
pub const HCI_DRV_READ_INFO_SIZE: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_drv_rp_read_info {
    pub driver_name: [__u8; HCI_DRV_MAX_DRIVER_NAME_LENGTH],
    pub num_supported_commands: __le16,
    pub __counted_by_le(num_supported_commands): __le16 supported_commands[],
    pub __packed: },
// Driver specific OGF (Opcode Group Field)
// Commands in this group may have different meanings across different drivers.
//
pub const HCI_DRV_OGF_DRIVER_SPECIFIC: c_uint = 0x01;
    pub status): *mut *mut int hci_drv_cmd_status(struct hci_dev hdev, u16 cmd, u8,
    pub rp_len): usize,
    pub cmd_skb): *mut *mut int hci_drv_process_cmd(struct hci_dev hdev, struct sk_buff,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_drv_handler {
    pub data_len): *mut *mut *mut *mut int (func)(struct hci_dev hdev, void data, u16,
    pub data_len: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_drv {
    pub common_handler_count: usize,
    pub common_handlers: *const hci_drv_handler,
    pub specific_handler_count: usize,
    pub specific_handlers: *const hci_drv_handler,
}
