//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtw89/usb.h
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
// Copyright(c) 2025  Realtek Corporation
//

pub const RTW89_USB_VENQT: c_uint = 0x05;
pub const RTW89_USB_VENQT_READ: c_uint = 0xc0;
pub const RTW89_USB_VENQT_WRITE: c_uint = 0x40;
pub const RTW89_USB_RECVBUF_SZ: c_int = 20480;
pub const RTW89_USB_RXCB_NUM: c_int = 8;
pub const RTW89_USB_RX_SKB_NUM: c_int = 16;
pub const RTW89_USB_MAX_RXQ_LEN: c_int = 512;
pub const RTW89_USB_MOD512_PADDING: c_int = 4;
pub const RTW89_MAX_ENDPOINT_NUM: c_int = 9;
pub const RTW89_MAX_BULKIN_NUM: c_int = 2;
pub const RTW89_MAX_BULKOUT_NUM: c_int = 7;
pub const R_AX_RXAGG_0_V1: c_uint = 0x6000;

pub const R_AX_RXAGG_1_V1: c_uint = 0x6004;
pub const R_BE_RXAGG_0_V1: c_uint = 0x6000;

pub const R_BE_RXAGG_1_V1: c_uint = 0x6004;
pub const R_AX_RXAGG_0: c_uint = 0x8900;

pub const RTW89_USB_MAX_TX_URBS_PER_CH: c_int = 128;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_usb_info {
    pub usb_host_request_2: u32,
    pub usb_wlan0_1: u32,
    pub hci_func_en: u32,
    pub usb3_mac_npi_config_intf_0: u32,
    pub usb_endpoint_0: u32,
    pub usb_endpoint_2: u32,
    pub rx_agg_alignment: u8,
    pub bulkout_id: [u8; RTW89_DMA_CH_NUM],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_usb_rx_ctrl_block {
    pub rtwdev: *mut rtw89_dev,
    pub rx_urb: *mut urb,
    pub rx_skb: *mut sk_buff,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_usb_tx_ctrl_block {
    pub rtwdev: *mut rtw89_dev,
    pub txch: u8,
    pub tx_ack_queue: sk_buff_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_usb {
    pub rtwdev: *mut rtw89_dev,
    pub udev: *mut usb_device,
    pub info: *const rtw89_usb_info,
    pub vendor_req_buf: *mut __le32,
    pub continual_io_error: core::sync::atomic::AtomicI32,
    pub in_pipe: [u8; RTW89_MAX_BULKIN_NUM],
    pub out_pipe: [u8; RTW89_MAX_BULKOUT_NUM],
    pub rxwq: *mut workqueue_struct,
    pub rx_cb: [rtw89_usb_rx_ctrl_block; RTW89_USB_RXCB_NUM],
    pub rx_queue: sk_buff_head,
    pub rx_free_queue: sk_buff_head,
    pub rx_work: work_struct,
    pub rx_urb_work: work_struct,
    pub tx_submitted: usb_anchor,
    pub tx_queue: [sk_buff_head; RTW89_TXCH_NUM],
    pub tx_inflight: [core::sync::atomic::AtomicI32; RTW89_TXCH_NUM],
}

extern "C" {
    pub fn rtw89_usb_disconnect(intf: *mut usb_interface);
}
