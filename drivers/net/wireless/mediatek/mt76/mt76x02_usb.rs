//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/mediatek/mt76/mt76x02_usb.h
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


// SPDX-License-Identifier: BSD-3-Clause-Clear
//
// Copyright (C) 2018 Lorenzo Bianconi <lorenzo.bianconi83@gmail.com>
//

// Macro flag: #define __MT76x02_USB_H

extern "C" {
    pub fn mt76x02u_mac_start(dev: *mut mt76x02_dev) -> c_int;
}
extern "C" {
    pub fn mt76x02u_init_mcu(dev: *mut mt76_dev);
}
extern "C" {
    pub fn mt76x02u_mcu_fw_reset(dev: *mut mt76x02_dev);
}
extern "C" {
    pub fn mt76x02u_skb_dma_info(skb: *mut sk_buff, port: c_int, flags: u32) -> c_int;
}
extern "C" {
    pub fn mt76x02u_tx_complete_skb(mdev: *mut mt76_dev, e: *mut mt76_queue_entry);
}
extern "C" {
    pub fn mt76x02u_init_beacon_config(dev: *mut mt76x02_dev);
}
extern "C" {
    pub fn mt76x02u_exit_beacon_config(dev: *mut mt76x02_dev);
}
