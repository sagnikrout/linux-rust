//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/bluetooth/hci_sock.h
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
// Socket options
pub const HCI_DATA_DIR: c_int = 1;
pub const HCI_FILTER: c_int = 2;
pub const HCI_TIME_STAMP: c_int = 3;
// CMSG flags
pub const HCI_CMSG_DIR: c_uint = 0x01;
pub const HCI_CMSG_TSTAMP: c_uint = 0x02;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr_hci {
    pub hci_family: sa_family_t,
    pub hci_dev: c_ushort,
    pub hci_channel: c_ushort,
}

pub const HCI_DEV_NONE: c_uint = 0xffff;
pub const HCI_CHANNEL_RAW: c_int = 0;
pub const HCI_CHANNEL_USER: c_int = 1;
pub const HCI_CHANNEL_MONITOR: c_int = 2;
pub const HCI_CHANNEL_CONTROL: c_int = 3;
pub const HCI_CHANNEL_LOGGING: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_filter {
    pub type_mask: c_ulong,
    pub event_mask: [c_ulong; 2],
    pub opcode: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_ufilter {
    pub type_mask: __u32,
    pub event_mask: [__u32; 2],
    pub opcode: __le16,
}

pub const HCI_FLT_TYPE_BITS: c_int = 31;
pub const HCI_FLT_EVENT_BITS: c_int = 63;
pub const HCI_FLT_OGF_BITS: c_int = 63;
pub const HCI_FLT_OCF_BITS: c_int = 127;
// Ioctl defines

// Ioctl requests structures
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_dev_stats {
    pub err_rx: __u32,
    pub err_tx: __u32,
    pub cmd_tx: __u32,
    pub evt_rx: __u32,
    pub acl_tx: __u32,
    pub acl_rx: __u32,
    pub sco_tx: __u32,
    pub sco_rx: __u32,
    pub byte_rx: __u32,
    pub byte_tx: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_dev_info {
    pub dev_id: __u16,
    pub name: [c_char; 8],
    pub bdaddr: bdaddr_t,
    pub flags: __u32,
    pub type: __u8,
    pub features: [__u8; 8],
    pub pkt_type: __u32,
    pub link_policy: __u32,
    pub link_mode: __u32,
    pub acl_mtu: __u16,
    pub acl_pkts: __u16,
    pub sco_mtu: __u16,
    pub sco_pkts: __u16,
    pub stat: hci_dev_stats,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_conn_info {
    pub handle: __u16,
    pub bdaddr: bdaddr_t,
    pub type: __u8,
    pub out: __u8,
    pub state: __u16,
    pub link_mode: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_dev_req {
    pub dev_id: __u16,
    pub dev_opt: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_dev_list_req {
    pub dev_num: __u16,
    pub __counted_by(dev_num): hci_dev_req dev_req[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_conn_list_req {
    pub dev_id: __u16,
    pub conn_num: __u16,
    pub conn_info: [hci_conn_info; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_conn_info_req {
    pub bdaddr: bdaddr_t,
    pub type: __u8,
    pub conn_info: [hci_conn_info; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_auth_info_req {
    pub bdaddr: bdaddr_t,
    pub type: __u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_inquiry_req {
    pub dev_id: __u16,
    pub flags: __u16,
    pub lap: [__u8; 3],
    pub length: __u8,
    pub num_rsp: __u8,
}

pub const IREQ_CACHE_FLUSH: c_uint = 0x0001;
