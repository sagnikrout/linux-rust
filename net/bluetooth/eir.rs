//! Automatically rewritten from C Header to Rust Module
//! Source: net/bluetooth/eir.h
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


// SPDX-License-Identifier: GPL-2.0
//
// BlueZ - Bluetooth protocol stack for Linux
//
// Copyright (C) 2021 Intel Corporation
//

extern "C" {
    pub fn eir_create(hdev: *mut hci_dev, data: *mut u8);
}
extern "C" {
    pub fn eir_create_adv_data(hdev: *mut hci_dev, instance: u8, ptr: *mut u8, size: u8) -> u8;
}
extern "C" {
    pub fn eir_create_scan_rsp(hdev: *mut hci_dev, instance: u8, ptr: *mut u8) -> u8;
}
extern "C" {
    pub fn eir_create_per_adv_data(hdev: *mut hci_dev, instance: u8, ptr: *mut u8) -> u8;
}
extern "C" {
    pub fn eir_append_local_name(hdev: *mut hci_dev, eir: *mut u8, ad_len: u8) -> u8;
}
extern "C" {
    pub fn eir_append_appearance(hdev: *mut hci_dev, ptr: *mut u8, ad_len: u8) -> u8;
}
// Zero length data
// data_len = field_len - 1;
