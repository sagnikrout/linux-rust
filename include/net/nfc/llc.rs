//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/nfc/llc.h
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
// Link Layer Control manager public interface
//
// Copyright (C) 2012  Intel Corporation. All rights reserved.
//

extern "C" {
    pub fn void(hdev: *mut *mut rcv_to_hci_t) (struct nfc_hci_dev, skb: *mut sk_buff) -> typedef;
}
extern "C" {
    pub fn int(hdev: *mut *mut xmit_to_drv_t) (struct nfc_hci_dev, skb: *mut sk_buff) -> typedef;
}
extern "C" {
    pub fn void(hdev: *mut *mut llc_failure_t) (struct nfc_hci_dev, err: c_int) -> typedef;
}
extern "C" {
    pub fn nfc_llc_free(llc: *mut nfc_llc);
}
extern "C" {
    pub fn nfc_llc_start(llc: *mut nfc_llc) -> c_int;
}
extern "C" {
    pub fn nfc_llc_stop(llc: *mut nfc_llc) -> c_int;
}
extern "C" {
    pub fn nfc_llc_rcv_from_drv(llc: *mut nfc_llc, skb: *mut sk_buff);
}
extern "C" {
    pub fn nfc_llc_xmit_from_hci(llc: *mut nfc_llc, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn nfc_llc_init() -> c_int;
}
extern "C" {
    pub fn nfc_llc_exit();
}
