//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/mediatek/mt7601u/usb.h
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
// Copyright (C) 2015 Jakub Kicinski <kubakici@wp.pl>
//

pub const MT_VEND_REQ_MAX_RETRY: c_int = 10;
pub const MT_VEND_REQ_TOUT_MS: c_int = 300;
pub const MT_VEND_DEV_MODE_RESET: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt_vendor_req {
    MT_VEND_DEV_MODE = 1,
    MT_VEND_WRITE = 2,
    MT_VEND_MULTI_READ = 7,
    MT_VEND_WRITE_FCE = 0x42,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt_usb_ep_in {
    MT_EP_IN_PKT_RX,
    MT_EP_IN_CMD_RESP,
    __MT_EP_IN_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt_usb_ep_out {
    MT_EP_OUT_INBAND_CMD,
    MT_EP_OUT_AC_BK,
    MT_EP_OUT_AC_BE,
    MT_EP_OUT_AC_VI,
    MT_EP_OUT_AC_VO,
    MT_EP_OUT_HCCA,
    __MT_EP_OUT_MAX,
}

extern "C" {
    pub fn interface_to_usbdev(_arg: to_usb_interface(mt7601u->dev)) -> return;
}
extern "C" {
    pub fn mt7601u_usb_free_buf(dev: *mut mt7601u_dev, buf: *mut mt7601u_dma_buf);
}
extern "C" {
    pub fn mt7601u_complete_urb(urb: *mut urb);
}
extern "C" {
    pub fn mt7601u_vendor_reset(dev: *mut mt7601u_dev);
}
