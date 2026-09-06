//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/broadcom/brcm80211/brcmfmac/msgbuf.h
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
// Copyright (c) 2014 Broadcom Corporation
//

pub const BRCMF_H2D_MSGRING_CONTROL_SUBMIT_MAX_ITEM: c_int = 64;
pub const BRCMF_H2D_MSGRING_RXPOST_SUBMIT_MAX_ITEM: c_int = 1024;
pub const BRCMF_D2H_MSGRING_CONTROL_COMPLETE_MAX_ITEM: c_int = 64;
pub const BRCMF_D2H_MSGRING_TX_COMPLETE_MAX_ITEM: c_int = 1024;
pub const BRCMF_D2H_MSGRING_RX_COMPLETE_MAX_ITEM: c_int = 1024;
pub const BRCMF_H2D_TXFLOWRING_MAX_ITEM: c_int = 512;
pub const BRCMF_H2D_MSGRING_CONTROL_SUBMIT_ITEMSIZE: c_int = 40;
pub const BRCMF_H2D_MSGRING_RXPOST_SUBMIT_ITEMSIZE: c_int = 32;
pub const BRCMF_D2H_MSGRING_CONTROL_COMPLETE_ITEMSIZE: c_int = 24;
pub const BRCMF_D2H_MSGRING_TX_COMPLETE_ITEMSIZE_PRE_V7: c_int = 16;
pub const BRCMF_D2H_MSGRING_TX_COMPLETE_ITEMSIZE: c_int = 24;
pub const BRCMF_D2H_MSGRING_RX_COMPLETE_ITEMSIZE_PRE_V7: c_int = 32;
pub const BRCMF_D2H_MSGRING_RX_COMPLETE_ITEMSIZE: c_int = 40;
pub const BRCMF_H2D_TXFLOWRING_ITEMSIZE: c_int = 48;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msgbuf_buf_addr {
    pub low_addr: __le32,
    pub high_addr: __le32,
}

extern "C" {
    pub fn brcmf_proto_msgbuf_rx_trigger(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn brcmf_msgbuf_delete_flowring(drvr: *mut brcmf_pub, flowid: u16);
}
extern "C" {
    pub fn brcmf_proto_msgbuf_attach(drvr: *mut brcmf_pub) -> c_int;
}
extern "C" {
    pub fn brcmf_proto_msgbuf_detach(drvr: *mut brcmf_pub);
}

