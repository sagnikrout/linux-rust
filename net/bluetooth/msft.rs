//! Automatically rewritten from C Header to Rust Module
//! Source: net/bluetooth/msft.h
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
// Copyright (C) 2020 Google Corporation
//

extern "C" {
    pub fn msft_monitor_supported(hdev: *mut hci_dev) -> bool;
}
extern "C" {
    pub fn msft_register(hdev: *mut hci_dev);
}
extern "C" {
    pub fn msft_release(hdev: *mut hci_dev);
}
extern "C" {
    pub fn msft_do_open(hdev: *mut hci_dev);
}
extern "C" {
    pub fn msft_do_close(hdev: *mut hci_dev);
}
extern "C" {
    pub fn msft_vendor_evt(hdev: *mut hci_dev, data: *mut c_void, skb: *mut sk_buff);
}
extern "C" {
    pub fn msft_get_features(hdev: *mut hci_dev) -> __u64;
}
extern "C" {
    pub fn msft_add_monitor_pattern(hdev: *mut hci_dev, monitor: *mut adv_monitor) -> c_int;
}
extern "C" {
    pub fn msft_remove_monitor(hdev: *mut hci_dev, monitor: *mut adv_monitor) -> c_int;
}
extern "C" {
    pub fn msft_req_add_set_filter_enable(req: *mut hci_request, enable: bool);
}
extern "C" {
    pub fn msft_set_filter_enable(hdev: *mut hci_dev, enable: bool) -> c_int;
}
extern "C" {
    pub fn msft_suspend_sync(hdev: *mut hci_dev) -> c_int;
}
extern "C" {
    pub fn msft_resume_sync(hdev: *mut hci_dev) -> c_int;
}
extern "C" {
    pub fn msft_curve_validity(hdev: *mut hci_dev) -> bool;
}

