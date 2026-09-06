//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/bluetooth/btmtk.h
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


// SPDX-License-Identifier: ISC
// Copyright (C) 2021 MediaTek Inc.

pub const HCI_EV_WMT: c_uint = 0xe4;
pub const HCI_WMT_MAX_EVENT_SIZE: c_int = 64;
pub const BTMTK_WMT_REG_WRITE: c_uint = 0x1;
pub const BTMTK_WMT_REG_READ: c_uint = 0x2;
pub const MT7921_BTSYS_RST: c_uint = 0x70002610;

pub const MT7921_PINMUX_0: c_uint = 0x70005050;
pub const MT7921_PINMUX_1: c_uint = 0x70005054;
pub const MT7921_DLSTATUS: c_uint = 0x7c053c10;

pub const MTK_COREDUMP_NUM: c_int = 255;
// UHW CR mapping
pub const MTK_BT_MISC: c_uint = 0x70002510;
pub const MTK_BT_SUBSYS_RST: c_uint = 0x70002610;
pub const MTK_UDMA_INT_STA_BT: c_uint = 0x74000024;
pub const MTK_UDMA_INT_STA_BT1: c_uint = 0x74000308;
pub const MTK_BT_WDT_STATUS: c_uint = 0x740003A0;
pub const MTK_EP_RST_OPT: c_uint = 0x74011890;
pub const MTK_EP_RST_IN_OUT_OPT: c_uint = 0x00010001;
pub const MTK_BT_RST_DONE: c_uint = 0x00000100;
pub const MTK_BT_RESET_REG_CONNV3: c_uint = 0x70028610;
pub const MTK_BT_READ_DEV_ID: c_uint = 0x70010200;
// MediaTek ISO Interface
pub const MTK_ISO_IFNUM: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btmtk_wmt_hdr {
    pub dir: u8,
    pub op: u8,
    pub dlen: __le16,
    pub flag: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btmtk_hci_wmt_cmd {
    pub hdr: btmtk_wmt_hdr,
    pub data: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btmtk_hci_wmt_evt {
    pub hhdr: hci_event_hdr,
    pub whdr: btmtk_wmt_hdr,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btmtk_hci_wmt_evt_funcc {
    pub hwhdr: btmtk_hci_wmt_evt,
    pub status: __be16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btmtk_hci_wmt_evt_reg {
    pub hwhdr: btmtk_hci_wmt_evt,
    pub rsv: [u8; 2],
    pub num: u8,
    pub addr: __le32,
    pub val: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btmtk_tci_sleep {
    pub mode: u8,
    pub duration: __le16,
    pub host_duration: __le16,
    pub host_wakeup_pin: u8,
    pub time_compensation: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btmtk_wakeon {
    pub mode: u8,
    pub gpo: u8,
    pub active_high: u8,
    pub enable_delay: __le16,
    pub wakeup_delay: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btmtk_sco {
    pub clock_config: u8,
    pub transmit_format_config: u8,
    pub channel_format_config: u8,
    pub channel_select_config: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct reg_read_cmd {
    pub type: u8,
    pub rsv: u8,
    pub num: u8,
    pub addr: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct reg_write_cmd {
    pub type: u8,
    pub rsv: u8,
    pub num: u8,
    pub addr: __le32,
    pub data: __le32,
    pub mask: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btmtk_hci_wmt_params {
    pub op: u8,
    pub flag: u8,
    pub dlen: u16,
    pub data: *const c_void,
    pub status: *mut u32,
}

extern "C" {
    pub fn int(: *mut *mut btmtk_reset_sync_func_t)(struct hci_dev, : *mut c_void) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btmtk_coredump_info {
    pub driver_name: *const c_char,
    pub fw_version: u32,
    pub cnt: u16,
    pub state: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btmtk_data {
    pub drv_name: *const c_char,
    pub flags: c_ulong,
    pub dev_id: u32,
    pub reset_sync: btmtk_reset_sync_func_t,
    pub cd_info: btmtk_coredump_info,
    pub udev: *mut usb_device,
    pub intf: *mut usb_interface,
    pub ctrl_anchor: *mut usb_anchor,
    pub evt_skb: *mut sk_buff,
    pub isopkt_tx_ep: *mut usb_endpoint_descriptor,
    pub isopkt_rx_ep: *mut usb_endpoint_descriptor,
    pub isopkt_intf: *mut usb_interface,
    pub isopkt_anchor: usb_anchor,
    pub isopkt_skb: *mut sk_buff,
// spinlock for ISO data transmission
    pub isorxlock: spinlock_t,
}

extern "C" {
    pub fn btmtk_set_bdaddr(hdev: *mut hci_dev, bdaddr: *const bdaddr_t) -> c_int;
}
extern "C" {
    pub fn btmtk_reset_sync(hdev: *mut hci_dev);
}
extern "C" {
    pub fn btmtk_process_coredump(hdev: *mut hci_dev, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn btmtk_usb_subsys_reset(hdev: *mut hci_dev, dev_id: u32) -> c_int;
}
extern "C" {
    pub fn btmtk_usb_recv_acl(hdev: *mut hci_dev, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn btmtk_usb_resume(hdev: *mut hci_dev) -> c_int;
}
extern "C" {
    pub fn btmtk_usb_suspend(hdev: *mut hci_dev) -> c_int;
}
extern "C" {
    pub fn btmtk_usb_setup(hdev: *mut hci_dev) -> c_int;
}
extern "C" {
    pub fn btmtk_usb_shutdown(hdev: *mut hci_dev) -> c_int;
}
extern "C" {
    pub fn btmtk_recv_event(hdev: *mut hci_dev, skb: *mut sk_buff) -> c_int;
}

extern "C" {
    pub fn ERR_PTR(_arg: -EOPNOTSUPP) -> return;
}
extern "C" {
    pub fn hci_recv_frame(_arg: hdev, _arg: skb) -> return;
}
