//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/nfc/nfcmrvl/nfcmrvl.h
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
// Marvell NFC driver
//
// Copyright (C) 2014-2015, Marvell International Ltd.
//

// Define private flags:
pub const NFCMRVL_NCI_RUNNING: c_int = 1;
pub const NFCMRVL_PHY_ERROR: c_int = 2;
pub const NFCMRVL_EXT_COEX_ID: c_uint = 0xE0;
pub const NFCMRVL_NOT_ALLOWED_ID: c_uint = 0xE1;
pub const NFCMRVL_ACTIVE_ID: c_uint = 0xE2;
pub const NFCMRVL_EXT_COEX_ENABLE: c_int = 1;
pub const NFCMRVL_GPIO_PIN_NFC_NOT_ALLOWED: c_uint = 0xA;
pub const NFCMRVL_GPIO_PIN_NFC_ACTIVE: c_uint = 0xB;
pub const NFCMRVL_NCI_MAX_EVENT_SIZE: c_int = 260;
//
// NCI FW Parameters
//
pub const NFCMRVL_PB_BAIL_OUT: c_uint = 0x11;
pub const NFCMRVL_PROP_REF_CLOCK: c_uint = 0xF0;
pub const NFCMRVL_PROP_SET_HI_CONFIG: c_uint = 0xF1;
//
// HCI defines
//
pub const NFCMRVL_HCI_EVENT_HEADER_SIZE: c_uint = 0x04;
pub const NFCMRVL_HCI_EVENT_CODE: c_uint = 0x04;
pub const NFCMRVL_HCI_NFC_EVENT_CODE: c_uint = 0xFF;
pub const NFCMRVL_HCI_COMMAND_CODE: c_uint = 0x01;
pub const NFCMRVL_HCI_OGF: c_uint = 0x81;
pub const NFCMRVL_HCI_OCF: c_uint = 0xFE;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfcmrvl_phy {
    NFCMRVL_PHY_USB		= 0,
    NFCMRVL_PHY_UART	= 1,
    NFCMRVL_PHY_I2C		= 2,
    NFCMRVL_PHY_SPI		= 3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfcmrvl_platform_data {
//
// Generic
//
// GPIO that is wired to RESET_N signal
    pub reset_gpio: *mut gpio_desc,
// Tell if transport is muxed in HCI one
    pub hci_muxed: bool,
//
// UART specific
//
// Tell if UART needs flow control at init
    pub flow_control: bool,
// Tell if firmware supports break control for power management
    pub break_control: bool,
//
// I2C specific
//
    pub irq: c_uint,
    pub irq_polarity: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfcmrvl_private {
    pub flags: c_ulong,
// Platform configuration
    pub config: nfcmrvl_platform_data,
// Parent dev
    pub ndev: *mut nci_dev,
// FW download context
    pub fw_dnld: nfcmrvl_fw_dnld,
// FW download support
    pub support_fw_dnld: bool,
//
// PHY related information
//
// PHY driver context
    pub drv_data: *mut c_void,
// PHY device
    pub dev: *mut device,
// PHY type
    pub phy: nfcmrvl_phy,
// Low level driver ops
    pub if_ops: *const nfcmrvl_if_ops,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfcmrvl_if_ops {
    pub priv): *mut *mut int (nci_open) (struct nfcmrvl_private,
    pub priv): *mut *mut int (nci_close) (struct nfcmrvl_private,
    pub skb): *mut *mut *mut int (nci_send) (struct nfcmrvl_private priv, struct sk_buff,
    pub param): *const c_void,
}

extern "C" {
    pub fn nfcmrvl_nci_unregister_dev(priv: *mut nfcmrvl_private);
}
extern "C" {
    pub fn nfcmrvl_nci_recv_frame(priv: *mut nfcmrvl_private, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn nfcmrvl_chip_reset(priv: *mut nfcmrvl_private);
}
extern "C" {
    pub fn nfcmrvl_chip_halt(priv: *mut nfcmrvl_private);
}
