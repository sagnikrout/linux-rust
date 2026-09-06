//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtw88/usb.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
// Copyright(c) 2018-2019  Realtek Corporation
//
pub const FW_8192C_START_ADDRESS: c_uint = 0x1000;
pub const FW_8192C_END_ADDRESS: c_uint = 0x5fff;
pub const RTW_USB_MAX_RXTX_COUNT: c_int = 128;
pub const RTW_USB_VENQT_MAX_BUF_SIZE: c_int = 254;
pub const MAX_USBCTRL_VENDORREQ_TIMES: c_int = 10;
pub const RTW_USB_CMD_READ: c_uint = 0xc0;
pub const RTW_USB_CMD_WRITE: c_uint = 0x40;
pub const RTW_USB_CMD_REQ: c_uint = 0x05;
pub const RTW_USB_VENQT_CMD_IDX: c_uint = 0x00;

pub const RTW_USB_BULK_IN_ADDR: c_uint = 0x80;
pub const RTW_USB_INT_IN_ADDR: c_uint = 0x81;
pub const RTW_USB_HW_QUEUE_ENTRY: c_int = 8;
pub const RTW_USB_PACKET_OFFSET_SZ: c_int = 8;

pub const RTW_USB_MAX_RECVBUF_SZ: c_int = 32768;
pub const RTW_USB_RECVBUFF_ALIGN_SZ: c_int = 8;
pub const RTW_USB_RXAGG_SIZE: c_int = 6;
pub const RTW_USB_RXAGG_TIMEOUT: c_int = 10;
pub const RTW_USB_RXCB_NUM: c_int = 4;
pub const RTW_USB_RX_SKB_NUM: c_int = 8;
pub const RTW_USB_EP_MAX: c_int = 4;
pub const TX_DESC_QSEL_MAX: c_int = 20;
pub const RTW_USB_VENDOR_ID_REALTEK: c_uint = 0x0bda;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rx_usb_ctrl_block {
    pub rtwdev: *mut rtw_dev,
    pub rx_urb: *mut urb,
    pub rx_skb: *mut sk_buff,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_usb_tx_data {
    pub sn: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_usb {
    pub rtwdev: *mut rtw_dev,
    pub udev: *mut usb_device,
// protects usb_data_index
    pub usb_lock: spinlock_t,
    pub usb_data: *mut __le32,
    pub usb_data_index: c_uint,
    pub pipe_interrupt: u8,
    pub pipe_in: u8,
    pub out_ep: [u8; RTW_USB_EP_MAX],
    pub qsel_to_ep: [c_int; TX_DESC_QSEL_MAX],
    pub rxwq: *mut *mut workqueue_txwq,,
    pub tx_queue: [sk_buff_head; RTW_USB_EP_MAX],
    pub tx_work: work_struct,
    pub rx_cb: [rx_usb_ctrl_block; RTW_USB_RXCB_NUM],
    pub rx_queue: sk_buff_head,
    pub rx_free_queue: sk_buff_head,
    pub rx_work: work_struct,
    pub rx_urb_work: work_struct,
}

extern "C" {
    pub fn rtw_usb_probe(intf: *mut usb_interface, id: *const usb_device_id) -> c_int;
}
extern "C" {
    pub fn rtw_usb_disconnect(intf: *mut usb_interface);
}
