//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/can/usb/peak_usb/pcan_usb_pro.h
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
// CAN driver for PEAK System PCAN-USB Pro adapter
// Derived from the PCAN project file driver/src/pcan_usbpro_fw.h
//
// Copyright (C) 2003-2025 PEAK System-Technik GmbH
// Author: Stéphane Grosjean <s.grosjean@peak-system.fr>
//
// USB Vendor request data types
//
pub const PCAN_USBPRO_REQ_INFO: c_int = 0;
pub const PCAN_USBPRO_REQ_FCT: c_int = 2;
// Vendor Request value for XXX_INFO
pub const PCAN_USBPRO_INFO_BL: c_int = 0;
pub const PCAN_USBPRO_INFO_FW: c_int = 1;
// PCAN-USB Pro (FD) Endpoints
pub const PCAN_USBPRO_EP_CMDOUT: c_int = 1;

pub const PCAN_USBPRO_EP_MSGOUT_0: c_int = 2;

pub const PCAN_USBPRO_EP_MSGOUT_1: c_int = 3;

// Vendor Request value for XXX_FCT

pub const PCAN_USBPRO_FCT_DRVLD_REQ_LEN: c_int = 16;
// PCAN_USBPRO_INFO_BL vendor request record type
// PCAN_USBPRO_INFO_FW vendor request record type
//
// USB Command record types
//
pub const PCAN_USBPRO_SETBTR: c_uint = 0x02;
pub const PCAN_USBPRO_SETBUSACT: c_uint = 0x04;
pub const PCAN_USBPRO_SETSILENT: c_uint = 0x05;
pub const PCAN_USBPRO_SETDEVID: c_uint = 0x06;
pub const PCAN_USBPRO_SETFILTR: c_uint = 0x0a;
pub const PCAN_USBPRO_SETTS: c_uint = 0x10;
pub const PCAN_USBPRO_GETDEVID: c_uint = 0x12;
pub const PCAN_USBPRO_SETLED: c_uint = 0x1C;
pub const PCAN_USBPRO_RXMSG8: c_uint = 0x80;
pub const PCAN_USBPRO_RXMSG4: c_uint = 0x81;
pub const PCAN_USBPRO_RXMSG0: c_uint = 0x82;
pub const PCAN_USBPRO_RXRTR: c_uint = 0x83;
pub const PCAN_USBPRO_RXSTATUS: c_uint = 0x84;
pub const PCAN_USBPRO_RXTS: c_uint = 0x85;
pub const PCAN_USBPRO_TXMSG8: c_uint = 0x41;
pub const PCAN_USBPRO_TXMSG4: c_uint = 0x42;
pub const PCAN_USBPRO_TXMSG0: c_uint = 0x43;
// record structures
pub const PCAN_USBPRO_LED_DEVICE: c_uint = 0x00;
pub const PCAN_USBPRO_LED_BLINK_FAST: c_uint = 0x01;
pub const PCAN_USBPRO_LED_BLINK_SLOW: c_uint = 0x02;
pub const PCAN_USBPRO_LED_ON: c_uint = 0x03;
pub const PCAN_USBPRO_LED_OFF: c_uint = 0x04;
pub const PCAN_USBPRO_STATUS_ERROR: c_uint = 0x0001;
pub const PCAN_USBPRO_STATUS_BUS: c_uint = 0x0002;
pub const PCAN_USBPRO_STATUS_OVERRUN: c_uint = 0x0004;
pub const PCAN_USBPRO_STATUS_QOVERRUN: c_uint = 0x0008;
#[repr(C)]
#[derive(Copy, Clone)]
pub union pcan_usb_pro_rec {
    pub data_type: u8,
    pub btr: pcan_usb_pro_btr,
    pub bus_act: pcan_usb_pro_busact,
    pub silent_mode: pcan_usb_pro_silent,
    pub filter_mode: pcan_usb_pro_filter,
    pub ts: pcan_usb_pro_setts,
    pub dev_id: pcan_usb_pro_devid,
    pub set_led: pcan_usb_pro_setled,
    pub rx_msg: pcan_usb_pro_rxmsg,
    pub rx_status: pcan_usb_pro_rxstatus,
    pub rx_ts: pcan_usb_pro_rxts,
    pub tx_msg: pcan_usb_pro_txmsg,
}

extern "C" {
    pub fn pcan_usb_pro_probe(intf: *mut usb_interface) -> c_int;
}
extern "C" {
    pub fn pcan_usb_pro_restart_complete(urb: *mut urb);
}
