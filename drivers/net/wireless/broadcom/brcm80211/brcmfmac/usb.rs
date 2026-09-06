//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/broadcom/brcm80211/brcmfmac/usb.h
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
//
// Copyright (c) 2011 Broadcom Corporation
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum brcmf_usb_state {
    BRCMFMAC_USB_STATE_DOWN,
    BRCMFMAC_USB_STATE_DL_FAIL,
    BRCMFMAC_USB_STATE_DL_DONE,
    BRCMFMAC_USB_STATE_UP,
    BRCMFMAC_USB_STATE_SLEEP
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_stats {
    pub tx_ctlpkts: u32,
    pub tx_ctlerrs: u32,
    pub rx_ctlpkts: u32,
    pub rx_ctlerrs: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_usbdev {
    pub bus: *mut brcmf_bus,
    pub devinfo: *mut brcmf_usbdev_info,
    pub state: brcmf_usb_state,
    pub stats: brcmf_stats,
    pub rxsize: int ntxq, nrxq,,
    pub bus_mtu: u32,
    pub devid: c_int,
    pub /: *mut *mut int chiprev; / chip revision number,
}

// IO Request Block (IRB)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_usbreq {
    pub list: list_head,
    pub devinfo: *mut brcmf_usbdev_info,
    pub urb: *mut urb,
    pub skb: *mut sk_buff,
}
